use anyhow::Result;
use std::collections::HashMap;
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

    let mut base_groups: HashMap<String, Vec<StackedPR>> = HashMap::new();
    
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

        base_groups
            .entry(format!("{}:{}", repo_name, base_branch))
            .or_default()
            .push(stacked_pr);
    }

    let mut stacks = Vec::new();
    for (repo_base, prs) in base_groups {
        if prs.len() < 2 {
            continue;
        }

        let mut prs = prs;
        prs.sort_by(|a, b| a.head_branch.cmp(&b.head_branch));
        
        let mut detected_stack: Vec<StackedPR> = Vec::new();

        for (i, pr) in prs.iter().enumerate() {
            if i == 0 {
                detected_stack.push(pr.clone());
                continue;
            }

            let prev_head = &prs[i - 1].head_branch;
            
            if is_stack_child(prev_head, &pr.head_branch) {
                detected_stack.push(pr.clone());
            } else {
                if detected_stack.len() >= 2 {
                    let (repo, base_branch) = repo_base.split_once(':').unwrap();
                    stacks.push(build_stack(repo, base_branch, detected_stack));
                }
                detected_stack = vec![pr.clone()];
            }
        }

        if detected_stack.len() >= 2 {
            let (repo, base_branch) = repo_base.split_once(':').unwrap();
            stacks.push(build_stack(repo, base_branch, detected_stack));
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

fn is_stack_child(parent: &str, child: &str) -> bool {
    child.starts_with(&format!("{}-", parent))
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
