use anyhow::Result;
use chrono::{DateTime, Utc};
use octocrab::Octocrab;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PendingReview {
    pub repo: String,
    pub pr_number: u64,
    pub pr_title: String,
    pub pr_author: String,
    pub pr_url: String,
    pub created_at: DateTime<Utc>,
    pub additions: u64,
    pub deletions: u64,
    pub draft: bool,
    pub branch: String,
}

pub async fn fetch_pr_by_number(
    token: &str,
    org: &str,
    repos: &[String],
    pr_number: u64,
) -> Result<Vec<PendingReview>> {
    let client = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let mut results = vec![];

    for repo in repos {
        match client.pulls(org, repo).get(pr_number).await {
            Ok(pr) => {
                let author = pr.user.as_ref().map(|u| u.login.clone()).unwrap_or_default();
                results.push(PendingReview {
                    repo: repo.clone(),
                    pr_number: pr.number,
                    pr_title: pr.title.clone().unwrap_or_default(),
                    pr_author: author,
                    pr_url: pr.html_url.map(|u| u.to_string()).unwrap_or_default(),
                    created_at: pr.created_at.unwrap_or_default(),
                    additions: pr.additions.unwrap_or(0) as u64,
                    deletions: pr.deletions.unwrap_or(0) as u64,
                    draft: pr.draft.unwrap_or(false),
                    branch: pr.head.label.clone().unwrap_or_default(),
                });
            }
            Err(_) => continue,
        }
    }

    Ok(results)
}

pub async fn fetch_pending_reviews(
    token: &str,
    org: &str,
    repos: &[String],
    username: &str,
    teams: &[String],
    include_mine: bool,
    include_drafts: bool,
    exclude_prefixes: &[String],
    crew_members: &[String],
) -> Result<Vec<PendingReview>> {
    let client = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let mut pending = vec![];

    for repo in repos {
        let prs = client
            .pulls(org, repo)
            .list()
            .state(octocrab::params::State::Open)
            .per_page(50)
            .send()
            .await?;

        for pr in prs.items {
            if !include_drafts && pr.draft.unwrap_or(false) {
                continue;
            }

            let title = pr.title.clone().unwrap_or_default();
            if exclude_prefixes.iter().any(|p| !p.is_empty() && title.starts_with(p)) {
                continue;
            }

            let author = pr.user.as_ref().map(|u| u.login.as_str()).unwrap_or("");

            // --crew mode: only show PRs authored by crew members
            if !crew_members.is_empty() {
                if !crew_members.iter().any(|m| m == author) {
                    continue;
                }
            } else {
                if !include_mine && author == username {
                    continue;
                }

                let user_requested = pr
                    .requested_reviewers
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .any(|r| r.login == username);

                let team_requested = if teams.is_empty() {
                    false
                } else {
                    pr.requested_teams
                        .as_deref()
                        .unwrap_or(&[])
                        .iter()
                        .any(|t| teams.contains(&t.slug.to_lowercase()))
                };

                if !user_requested && !team_requested {
                    continue;
                }
            }

            // Fetch individual PR to get diff stats
            let detail = client.pulls(org, repo).get(pr.number).await?;

            pending.push(PendingReview {
                repo: repo.clone(),
                pr_number: pr.number,
                pr_title: title.clone(),
                pr_author: author.to_string(),
                pr_url: pr.html_url.map(|u| u.to_string()).unwrap_or_default(),
                created_at: pr.created_at.unwrap_or_default(),
                additions: detail.additions.unwrap_or(0) as u64,
                deletions: detail.deletions.unwrap_or(0) as u64,
                draft: pr.draft.unwrap_or(false),
                branch: pr.head.label.clone().unwrap_or_default(),
            });
        }
    }

    // Sort: oldest first (most urgent)
    pending.sort_by_key(|r| r.created_at);
    Ok(pending)
}

pub async fn fetch_my_open_prs(
    token: &str,
    org: &str,
    repos: &[String],
    username: &str,
    include_drafts: bool,
    exclude_prefixes: &[String],
) -> Result<Vec<PendingReview>> {
    let client = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let mut my_prs = vec![];

    for repo in repos {
        let prs = client
            .pulls(org, repo)
            .list()
            .state(octocrab::params::State::Open)
            .per_page(50)
            .send()
            .await?;

        for pr in prs.items {
            if !include_drafts && pr.draft.unwrap_or(false) {
                continue;
            }

            let title = pr.title.clone().unwrap_or_default();
            if exclude_prefixes.iter().any(|p| !p.is_empty() && title.starts_with(p)) {
                continue;
            }

            let author = pr.user.as_ref().map(|u| u.login.as_str()).unwrap_or("");
            if author != username {
                continue;
            }

            // Fetch individual PR to get diff stats
            let detail = client.pulls(org, repo).get(pr.number).await?;

            my_prs.push(PendingReview {
                repo: repo.clone(),
                pr_number: pr.number,
                pr_title: title.clone(),
                pr_author: author.to_string(),
                pr_url: pr.html_url.map(|u| u.to_string()).unwrap_or_default(),
                created_at: pr.created_at.unwrap_or_default(),
                additions: detail.additions.unwrap_or(0) as u64,
                deletions: detail.deletions.unwrap_or(0) as u64,
                draft: pr.draft.unwrap_or(false),
                branch: pr.head.label.clone().unwrap_or_default(),
            });
        }
    }

    // Sort: oldest first (most urgent)
    my_prs.sort_by_key(|r| r.created_at);
    Ok(my_prs)
}

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PullRequestFile {
    pub filename: String,
    pub status: String,
    pub additions: u64,
    pub deletions: u64,
}

pub async fn fetch_pr_files(
    token: &str,
    org: &str,
    repo: &str,
    pr_number: u64,
) -> Result<Vec<PullRequestFile>> {
    let client = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let files: Vec<PullRequestFile> = client
        .pulls(org, repo)
        .list_files(pr_number)
        .await?
        .into_iter()
        .map(|f| {
            let status_str = match f.status {
                octocrab::models::repos::DiffEntryStatus::Added => "added",
                octocrab::models::repos::DiffEntryStatus::Removed => "removed",
                octocrab::models::repos::DiffEntryStatus::Modified => "modified",
                octocrab::models::repos::DiffEntryStatus::Renamed => "renamed",
                octocrab::models::repos::DiffEntryStatus::Copied => "copied",
                octocrab::models::repos::DiffEntryStatus::Changed => "changed",
                octocrab::models::repos::DiffEntryStatus::Unchanged => "unchanged",
                _ => "unknown",
            };
            PullRequestFile {
                filename: f.filename,
                status: status_str.to_string(),
                additions: f.additions,
                deletions: f.deletions,
            }
        })
        .collect();

    Ok(files)
}

#[derive(Debug, Clone, Serialize)]
pub struct PullRequestLabel {
    pub id: String,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
}

pub async fn fetch_pr_labels(
    token: &str,
    org: &str,
    repo: &str,
    pr_number: u64,
) -> Result<Vec<PullRequestLabel>> {
    let client = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let labels: Vec<PullRequestLabel> = client
        .pulls(org, repo)
        .get(pr_number)
        .await?
        .labels
        .unwrap_or_default()
        .into_iter()
        .map(|l| PullRequestLabel {
            id: l.id.to_string(),
            name: l.name,
            color: l.color,
            description: l.description,
        })
        .collect();

    Ok(labels)
}

/// Represents a single file's diff with metadata.
#[derive(Debug, Clone)]
pub struct PullRequestDiff {
    pub filename: String,
    pub status: String, // "added", "removed", "modified", "renamed"
    pub additions: u64,
    pub deletions: u64,
    pub patch: Option<String>,
    pub language: Option<String>,
}

/// Represents merge conflict status for a PR
#[derive(Debug, Clone, Serialize)]
pub struct MergeConflictStatus {
    pub repo: String,
    pub pr_number: u64,
    pub pr_title: String,
    pub has_conflicts: bool,
    pub mergeable: Option<bool>,
    pub rebaseable: Option<bool>,
}

pub async fn fetch_pr_diff(
    token: &str,
    org: &str,
    repo: &str,
    pr_number: u64,
) -> Result<Vec<PullRequestDiff>> {
    let client = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let files: Vec<PullRequestDiff> = client
        .pulls(org, repo)
        .list_files(pr_number)
        .await?
        .into_iter()
        .map(|f| {
            let status_str = match f.status {
                octocrab::models::repos::DiffEntryStatus::Added => "added",
                octocrab::models::repos::DiffEntryStatus::Removed => "removed",
                octocrab::models::repos::DiffEntryStatus::Modified => "modified",
                octocrab::models::repos::DiffEntryStatus::Renamed => "renamed",
                octocrab::models::repos::DiffEntryStatus::Copied => "copied",
                octocrab::models::repos::DiffEntryStatus::Changed => "changed",
                octocrab::models::repos::DiffEntryStatus::Unchanged => "unchanged",
                _ => "unknown",
            };
            // Extract language from filename extension
            let language = f.filename.split('.').last().map(|ext| {
                match ext {
                    "ts" | "tsx" => "typescript",
                    "js" | "jsx" | "mjs" | "cjs" => "javascript",
                    "py" => "python",
                    "go" => "go",
                    "java" => "java",
                    "rb" => "ruby",
                    "cs" => "csharp",
                    "cpp" | "cc" | "cxx" => "cpp",
                    "c" | "h" => "c",
                    "swift" => "swift",
                    "kt" | "kts" => "kotlin",
                    "scala" => "scala",
                    "rs" => "rust",
                    "php" => "php",
                    "ex" | "exs" => "elixir",
                    "erl" => "erlang",
                    "hs" => "haskell",
                    "ml" | "mli" => "ocaml",
                    "fs" | "fsx" => "fsharp",
                    "lua" => "lua",
                    "r" => "r",
                    "sql" => "sql",
                    "sh" | "bash" | "zsh" => "bash",
                    "ps1" => "powershell",
                    "yml" | "yaml" => "yaml",
                    "toml" => "toml",
                    "json" => "json",
                    "xml" => "xml",
                    "html" | "htm" => "html",
                    "css" | "scss" | "sass" | "less" => "css",
                    "md" | "markdown" => "markdown",
                    "dockerfile" => "dockerfile",
                    "tf" => "hcl",
                    "proto" => "protobuf",
                    _ => ext,
                }.to_string()
            });
            PullRequestDiff {
                filename: f.filename,
                status: status_str.to_string(),
                additions: f.additions,
                deletions: f.deletions,
                patch: f.patch,
                language,
            }
        })
        .collect();

    Ok(files)
}

/// Represents a review event from GitHub history
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReviewActivity {
    pub repo: String,
    pub pr_number: u64,
    pub pr_title: String,
    pub author: String,
    pub reviewed_at: chrono::DateTime<chrono::Utc>,
    pub state: String, // "APPROVED", "CHANGES_REQUESTED", "COMMENTED"
}

pub async fn fetch_my_review_activity(
    token: &str,
    org: &str,
    repos: &[String],
    username: &str,
    days: u32,
) -> Result<Vec<ReviewActivity>> {
    let client = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let since = chrono::Utc::now() - chrono::Duration::days(days as i64);
    let mut all_reviews = vec![];

    for repo in repos {
        // List all PRs (open + closed) and check if the user reviewed them
        let prs = client
            .pulls(org, repo)
            .list()
            .state(octocrab::params::State::All)
            .per_page(100)
            .send()
            .await?;

        for pr in prs.items {
            // Check if this PR was updated within our window
            let updated_at = pr.updated_at
                .unwrap_or_else(|| pr.created_at.unwrap_or_else(chrono::Utc::now));
            if updated_at < since {
                continue;
            }

            // Get the timeline (includes reviews) for this PR
            let timeline: Vec<serde_json::Value> = client
                .get(
                    format!(
                        "/repos/{}/{}/issues/{}/timeline",
                        org, repo, pr.number
                    ),
                    None::<&str>,
                )
                .await
                .unwrap_or_default();

            for event in timeline {
                // Look for "PullRequestReview" events by our username
                if event.get("event").and_then(|e| e.as_str()) == Some("pull_request_review") {
                    if let Some(user_obj) = event.get("user") {
                        if user_obj.get("login").and_then(|l| l.as_str()) == Some(username) {
                            let reviewed_at = event
                                .get("submitted_at")
                                .and_then(|t| t.as_str())
                                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                                .map(|dt| dt.with_timezone(&chrono::Utc))
                                .unwrap_or(updated_at);

                            let state = event
                                .get("state")
                                .and_then(|s| s.as_str())
                                .unwrap_or("COMMENTED")
                                .to_string();

                            let author = pr.user.as_ref()
                                .map(|u| u.login.clone())
                                .unwrap_or_default();

                            all_reviews.push(ReviewActivity {
                                repo: repo.clone(),
                                pr_number: pr.number,
                                pr_title: pr.title.clone().unwrap_or_default(),
                                author,
                                reviewed_at,
                                state,
                            });
                            break; // one review per user per PR
                        }
                    }
                }
            }
        }
    }

    // Sort by most recent first
    all_reviews.sort_by(|a, b| b.reviewed_at.cmp(&a.reviewed_at));
    Ok(all_reviews)
}

/// Fetch merge conflict status for a list of pending reviews
pub async fn fetch_merge_conflict_status(
    token: &str,
    org: &str,
    reviews: &[PendingReview],
) -> Result<Vec<MergeConflictStatus>> {
    let client = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let mut results = Vec::new();

    // Fetch PR details to get mergeable status
    // Group by repo to minimize API calls per repo
    use std::collections::HashMap;
    let mut by_repo: HashMap<String, Vec<u64>> = HashMap::new();
    for review in reviews {
        by_repo.entry(review.repo.clone())
            .or_insert_with(Vec::new)
            .push(review.pr_number);
    }

    for (repo, pr_numbers) in by_repo {
        for pr_number in pr_numbers {
            match client.pulls(org, &repo).get(pr_number).await {
                Ok(pr) => {
                    let has_conflicts = pr.mergeable == Some(false);
                    let mergeable_status = MergeConflictStatus {
                        repo: repo.clone(),
                        pr_number,
                        pr_title: pr.title.unwrap_or_default(),
                        has_conflicts,
                        mergeable: pr.mergeable,
                        rebaseable: pr.rebaseable,
                    };
                    results.push(mergeable_status);
                }
                Err(e) => {
                    // Log but don't fail - just skip this PR
                    eprintln!("Warning: Failed to fetch PR #{} in {}: {}", pr_number, repo, e);
                }
            }
        }
    }

    Ok(results)
}
