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
