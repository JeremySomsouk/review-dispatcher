use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::process::Command;

#[derive(Clone)]
pub struct StackedPR {
    pub number: u64,
    pub title: String,
    pub repo: String,
    pub head_branch: String,
    pub base_branch: String,
    pub position: usize,
    pub url: String,
}

pub struct Stack {
    pub base_branch: String,
    pub repo: String,
    pub prs: Vec<StackedPR>,
}

/// Detect stacked PRs by analyzing open PRs and their branch relationships
pub fn detect_stacks(repo_filter: Option<&str>, limit: Option<u32>) -> Result<Vec<Stack>> {
    let output = Command::new("gh")
        .args([
            "pr", "list",
            "--state", "open",
            "--json", "number,title,repository,headRefName,baseRefName,url",
            "--limit", "100",
        ])
        .output()?;
    
    let json_output = String::from_utf8_lossy(&output.stdout);
    let prs_data: Vec<serde_json::Value> = serde_json::from_str(&json_output)?;

    let mut all_prs: Vec<StackedPR> = Vec::new();
    
    for pr in prs_data {
        let repo_name = pr["repository"]["name"].as_str().unwrap_or("");
        let head_branch = pr["headRefName"].as_str().unwrap_or("");
        let base_branch = pr["baseRefName"].as_str().unwrap_or("");
        let number = pr["number"].as_u64().unwrap_or(0);
        let title = pr["title"].as_str().unwrap_or("");
        let url = pr["url"].as_str().unwrap_or("");

        if let Some(filter) = repo_filter {
            if !repo_name.to_lowercase().contains(&filter.to_lowercase()) {
                continue;
            }
        }

        let stacked_pr = StackedPR {
            number,
            title: title.to_string(),
            repo: repo_name.to_string(),
            head_branch: head_branch.to_string(),
            base_branch: base_branch.to_string(),
            position: 0,
            url: url.to_string(),
        };

        all_prs.push(stacked_pr);
    }

    // Build a map of branch -> PR for quick lookup
    let mut branch_to_pr: HashMap<String, Vec<StackedPR>> = HashMap::new();
    for pr in &all_prs {
        branch_to_pr.entry(pr.head_branch.clone()).or_default().push(pr.clone());
    }

    let mut stacks = Vec::new();
    let mut used_prs: HashSet<u64> = HashSet::new();
    
    // Find stacks by checking if a PR's base branch is another PR's head branch
    for pr in &all_prs {
        if used_prs.contains(&pr.number) {
            continue;
        }
        
        // Check if this PR's base branch is another PR's head branch (meaning it's stacked)
        if let Some(parent_prs) = branch_to_pr.get(&pr.base_branch) {
            for parent_pr in parent_prs {
                if parent_pr.repo == pr.repo && parent_pr.number != pr.number {
                    // Found a stacked relationship
                    let mut stack = Vec::new();
                    let mut current_pr = parent_pr;
                    
                    // Build the stack upwards
                    while let Some(grandparent_prs) = branch_to_pr.get(&current_pr.base_branch) {
                        if let Some(grandparent_pr) = grandparent_prs.iter().find(|gp| 
                            gp.repo == current_pr.repo && gp.number != current_pr.number) {
                            stack.insert(0, grandparent_pr.clone());
                            used_prs.insert(grandparent_pr.number);
                            current_pr = grandparent_pr;
                        } else {
                            break;
                        }
                    }
                    
                    // Add the parent and current PR
                    stack.push(parent_pr.clone());
                    stack.push(pr.clone());
                    used_prs.insert(parent_pr.number);
                    used_prs.insert(pr.number);
                    
                    if stack.len() >= 2 {
                        let base_branch = stack.first().unwrap().base_branch.clone();
                        let repo = stack.first().unwrap().repo.clone();
                        stacks.push(build_stack(&repo, &base_branch, stack));
                    }
                    break;
                }
            }
        }
    }

    if let Some(l) = limit {
        stacks.truncate(l as usize);
    }

    Ok(stacks)
}

fn build_stack(repo: &str, base_branch: &str, prs: Vec<StackedPR>) -> Stack {
    let mut stack = Stack {
        base_branch: base_branch.to_string(),
        repo: repo.to_string(),
        prs,
    };
    for (i, pr) in stack.prs.iter_mut().enumerate() {
        pr.position = i + 1;
    }
    stack
}

pub fn render_stacks(stacks: &[Stack]) -> String {
    use colored::*;
    
    let mut output = String::new();
    
    if stacks.is_empty() {
        output.push_str("\n🔍 No stacked PRs detected.\n");
        output.push_str("Tip: Stacks are PRs with sequential branch names like:\n");
        output.push_str("  feature → feature-2 → feature-3\n");
        return output;
    }

    output.push_str(&format!("\n📦 Found {} stack(s)\n\n", stacks.len()));
    
    for stack in stacks {
        output.push_str(&format!("┌─ Stack on `{}` ({} PRs)\n", stack.base_branch, stack.prs.len()));
        
        for pr in &stack.prs {
            let prefix = if pr.position == 1 { "🔵" } else { "  " };
            output.push_str(&format!(
                "{} #{} - {} \n  └─ @{} \n    {}\n",
                prefix,
                pr.number,
                pr.title.bold(),
                pr.head_branch.dimmed(),
                pr.url.blue().underline()
            ));
        }
        output.push('\n');
    }

    output
}
