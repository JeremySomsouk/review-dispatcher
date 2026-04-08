use anyhow::Result;
use futures::future::join_all;
use octocrab::Octocrab;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct StackedPR {
    pub number: u64,
    pub title: String,
    pub repo: String,
    pub head_branch: String,
    pub base_branch: String,
    pub position: usize,
    pub url: String,
    pub author: String,
    pub draft: bool,
}

#[derive(Clone, Debug)]
pub struct Stack {
    pub base_branch: String,
    pub repo: String,
    pub prs: Vec<StackedPR>,
    pub kind: StackKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StackKind {
    /// PRs chained by branch: PR B targets PR A's head branch
    BranchChain,
    /// PRs grouped by ticket prefix + [N/M] markers in title
    Convention,
}

/// Detect stacked PRs using both branch-chaining and convention-based grouping
pub async fn detect_stacks(
    token: &str,
    org: &str,
    repos: &[String],
    author: Option<&str>,
    repo_filter: Option<&str>,
    include_drafts: bool,
    limit: Option<u32>,
) -> Result<Vec<Stack>> {
    let all_prs = fetch_all_prs(token, org, repos, author, repo_filter, include_drafts).await?;

    if all_prs.is_empty() {
        return Ok(Vec::new());
    }

    let mut stacks = Vec::new();
    let mut used_prs: HashSet<(String, u64)> = HashSet::new();

    // 1. Detect branch-chaining stacks
    let chain_stacks = detect_branch_chain_stacks(&all_prs);
    for stack in &chain_stacks {
        for pr in &stack.prs {
            used_prs.insert((pr.repo.clone(), pr.number));
        }
    }
    stacks.extend(chain_stacks);

    // 2. Detect convention-based stacks (ticket prefix + [N/M] markers)
    let convention_stacks = detect_convention_stacks(&all_prs, &used_prs);
    for stack in &convention_stacks {
        for pr in &stack.prs {
            used_prs.insert((pr.repo.clone(), pr.number));
        }
    }
    stacks.extend(convention_stacks);

    if let Some(l) = limit {
        stacks.truncate(l as usize);
    }

    Ok(stacks)
}

/// Fetch all open PRs across configured repos using octocrab
async fn fetch_all_prs(
    token: &str,
    org: &str,
    repos: &[String],
    author: Option<&str>,
    repo_filter: Option<&str>,
    include_drafts: bool,
) -> Result<Vec<StackedPR>> {
    let repo_futures = repos.iter().map(|repo_name| {
        let client = Octocrab::builder()
            .personal_token(token.to_string())
            .build()
            .unwrap();
        let repo_name = repo_name.clone();
        async move {
            client
                .pulls(org, &repo_name)
                .list()
                .state(octocrab::params::State::Open)
                .per_page(100)
                .send()
                .await
                .map(|prs| (repo_name, prs.items))
        }
    });

    let repo_results: Vec<(String, Vec<_>)> = join_all(repo_futures)
        .await
        .into_iter()
        .filter_map(|result| match result {
            Ok((repo, items)) => Some((repo, items)),
            Err(e) => {
                eprintln!("Warning: Failed to fetch PRs from a repo: {}", e);
                None
            }
        })
        .collect();

    let mut all_prs: Vec<StackedPR> = Vec::new();

    for (repo_name, prs) in repo_results {
        // Apply repo filter
        if let Some(filter) = repo_filter {
            if !repo_name.to_lowercase().contains(&filter.to_lowercase()) {
                continue;
            }
        }

        for pr in prs {
            let pr_author = pr.user.as_ref().map(|u| u.login.as_str()).unwrap_or("");

            // Apply author filter
            if let Some(author_filter) = author {
                if pr_author.to_lowercase() != author_filter.to_lowercase() {
                    continue;
                }
            }

            // Skip drafts unless included
            if !include_drafts && pr.draft.unwrap_or(false) {
                continue;
            }

            let head_branch = pr.head.ref_field.clone();
            let base_branch = pr.base.ref_field.clone();
            let title = pr.title.clone().unwrap_or_default();
            let url = pr.html_url.map(|u| u.to_string()).unwrap_or_default();

            all_prs.push(StackedPR {
                number: pr.number,
                title,
                repo: repo_name.clone(),
                head_branch,
                base_branch,
                position: 0,
                url,
                author: pr_author.to_string(),
                draft: pr.draft.unwrap_or(false),
            });
        }
    }

    // Sort by repo then PR number for consistency
    all_prs.sort_by(|a, b| {
        a.repo
            .cmp(&b.repo)
            .then_with(|| a.number.cmp(&b.number))
    });

    Ok(all_prs)
}

/// Detect stacks where a PR's base branch is another PR's head branch.
/// Uses a root-first approach: finds chain roots (PRs whose base is NOT another PR's head),
/// then walks down each chain to build the full stack without overlaps.
fn detect_branch_chain_stacks(all_prs: &[StackedPR]) -> Vec<Stack> {
    // Build maps for quick lookup
    // base_branch -> PRs that have this as their base (children)
    let mut base_to_children: HashMap<(String, String), Vec<&StackedPR>> = HashMap::new();
    // head_branch -> PRs that have this as their head (parents)
    let mut head_to_prs: HashMap<(String, String), Vec<&StackedPR>> = HashMap::new();

    for pr in all_prs {
        head_to_prs
            .entry((pr.repo.clone(), pr.head_branch.clone()))
            .or_default()
            .push(pr);
        base_to_children
            .entry((pr.repo.clone(), pr.base_branch.clone()))
            .or_default()
            .push(pr);
    }

    // Find chain roots: PRs whose base branch is NOT any other PR's head branch
    // (these are the starting points of chains)
    let roots: Vec<&StackedPR> = all_prs.iter().filter(|pr| {
        let key = (pr.repo.clone(), pr.base_branch.clone());
        // A root is a PR whose base branch isn't another PR's head branch
        !head_to_prs.contains_key(&key)
            || head_to_prs.get(&key).map_or(true, |prs| prs.iter().all(|p| p.number == pr.number))
    }).collect();

    let mut stacks = Vec::new();
    let mut used_prs: HashSet<(String, u64)> = HashSet::new();

    // Walk down from each root to build chains
    for root in roots {
        if used_prs.contains(&(root.repo.clone(), root.number)) {
            continue;
        }

        let mut chain = vec![root.clone()];
        used_prs.insert((root.repo.clone(), root.number));

        // Walk down: find PRs whose base_branch == last PR's head_branch
        loop {
            let last = chain.last().unwrap();
            let child_key = (last.repo.clone(), last.head_branch.clone());
            if let Some(children) = base_to_children.get(&child_key) {
                // Find a child that's not already used
                if let Some(child) = children.iter().find(|c| !used_prs.contains(&(c.repo.clone(), c.number)) && c.number != last.number) {
                    used_prs.insert((child.repo.clone(), child.number));
                    chain.push((*child).clone());
                    continue;
                }
            }
            break;
        }

        // Only create a stack if the chain has 2+ PRs
        if chain.len() >= 2 {
            let base_branch = chain.first().unwrap().base_branch.clone();
            let repo = chain.first().unwrap().repo.clone();
            stacks.push(build_stack(&repo, &base_branch, chain, StackKind::BranchChain));
        }
    }

    stacks
}

/// Extract a ticket key from a branch name or title.
/// Matches patterns like TAHC-1666, PROJ-123, ABC-1, etc.
fn extract_ticket_key(s: &str) -> Option<String> {
    let re = regex::Regex::new(r"(?i)\b([A-Z][A-Z0-9]*-\d+)\b").ok()?;
    let caps = re.captures(s)?;
    let key = caps.get(1)?.as_str().to_uppercase();
    Some(key)
}

/// Extract the position index from title markers like [1/3], (2/5), [3/3], etc.
fn extract_position_index(title: &str) -> Option<usize> {
    let re = regex::Regex::new(r"[\[\(](\d+)/(\d+)[\]\)]").ok()?;
    let caps = re.captures(title)?;
    caps.get(1)?.as_str().parse::<usize>().ok()
}

/// Detect convention-based stacks: PRs sharing the same ticket key
/// with [N/M] position markers in the title
fn detect_convention_stacks(all_prs: &[StackedPR], already_used: &HashSet<(String, u64)>) -> Vec<Stack> {
    // Group PRs by (repo, ticket_key) that have [N/M] markers
    let mut ticket_groups: HashMap<(String, String), Vec<&StackedPR>> = HashMap::new();

    for pr in all_prs {
        if already_used.contains(&(pr.repo.clone(), pr.number)) {
            continue;
        }

        // Try to extract ticket key from branch name first, then title
        let ticket_key = extract_ticket_key(&pr.head_branch)
            .or_else(|| extract_ticket_key(&pr.title));

        if let Some(key) = ticket_key {
            // Only include PRs that have a position marker [N/M] or are in a group with them
            ticket_groups
                .entry((pr.repo.clone(), key))
                .or_default()
                .push(pr);
        }
    }

    let mut stacks = Vec::new();

    for ((repo, ticket_key), mut prs) in ticket_groups {
        // Only consider groups with 2+ PRs
        if prs.len() < 2 {
            continue;
        }

        // Check if at least one PR has a [N/M] marker, or all share the same base branch
        let has_position_marker = prs.iter().any(|p| extract_position_index(&p.title).is_some());
        let all_same_base = prs.windows(2).all(|w| w[0].base_branch == w[1].base_branch);

        if !has_position_marker && !all_same_base {
            continue;
        }

        // Sort by position marker if available, otherwise by PR number
        prs.sort_by(|a, b| {
            let pos_a = extract_position_index(&a.title).unwrap_or(0);
            let pos_b = extract_position_index(&b.title).unwrap_or(0);
            pos_a.cmp(&pos_b).then_with(|| a.number.cmp(&b.number))
        });

        let stacked_prs: Vec<StackedPR> = prs.into_iter().cloned().collect();
        let base_branch = stacked_prs
            .first()
            .map(|p| p.base_branch.clone())
            .unwrap_or_default();

        // Include ticket key in the stack for display
        stacks.push(build_stack_with_key(
            &repo,
            &base_branch,
            &ticket_key,
            stacked_prs,
            StackKind::Convention,
        ));
    }

    stacks
}

fn build_stack(repo: &str, base_branch: &str, prs: Vec<StackedPR>, kind: StackKind) -> Stack {
    let mut stack = Stack {
        base_branch: base_branch.to_string(),
        repo: repo.to_string(),
        prs,
        kind,
    };
    for (i, pr) in stack.prs.iter_mut().enumerate() {
        pr.position = i + 1;
    }
    stack
}

fn build_stack_with_key(
    repo: &str,
    base_branch: &str,
    _ticket_key: &str,
    prs: Vec<StackedPR>,
    kind: StackKind,
) -> Stack {
    // For now, same as build_stack — ticket_key can be used for display later
    build_stack(repo, base_branch, prs, kind)
}

pub fn render_stacks(stacks: &[Stack]) -> String {
    use colored::*;

    let mut output = String::new();

    if stacks.is_empty() {
        output.push_str("\n🔍 No stacked PRs detected.\n");
        output.push_str("Tip: Stacks are detected via:\n");
        output.push_str("  • Branch chaining: PR B targets PR A's head branch\n");
        output.push_str("  • Convention: same ticket key (e.g. TAHC-1666) with [N/M] markers\n");
        return output;
    }

    output.push_str(&format!("\n📦 Found {} stack(s)\n\n", stacks.len()));

    for stack in stacks {
        let kind_label = match stack.kind {
            StackKind::BranchChain => "branch-chain",
            StackKind::Convention => "convention",
        };
        output.push_str(&format!(
            "┌─ Stack on `{}` ({} PRs, {})\n",
            stack.base_branch, stack.prs.len(), kind_label
        ));

        for pr in &stack.prs {
            let prefix = if pr.position == 1 { "🔵" } else { "  " };
            let draft_tag = if pr.draft { " [DRAFT]" } else { "" };
            output.push_str(&format!(
                "{} [{}/{}] #{} - {}{}\n  └─ {} \n    {}\n",
                prefix,
                pr.position,
                stack.prs.len(),
                pr.number,
                pr.title.bold(),
                draft_tag,
                pr.head_branch.dimmed(),
                pr.url.blue().underline()
            ));
        }
        output.push('\n');
    }

    output
}
