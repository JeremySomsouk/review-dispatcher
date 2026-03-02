use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Config {
    pub github_token: String,
    pub github_username: String,
    pub github_org: String,
    pub github_repos: Vec<String>,
    pub github_teams: Vec<String>,
    pub crew_members: Vec<String>,
    #[allow(dead_code)]
    pub anthropic_api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        Ok(Self {
            github_token: std::env::var("RD_GITHUB_TOKEN")
                .or_else(|_| std::env::var("GITHUB_TOKEN"))?,
            github_username: std::env::var("RD_GITHUB_USERNAME")
                .or_else(|_| std::env::var("GITHUB_USERNAME"))?,
            github_org: std::env::var("RD_GITHUB_ORG")
                .or_else(|_| std::env::var("GITHUB_ORG"))?,
            github_repos: std::env::var("RD_GITHUB_REPOS")
                .or_else(|_| std::env::var("GITHUB_REPOS"))
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            github_teams: std::env::var("RD_GITHUB_TEAMS")
                .or_else(|_| std::env::var("GITHUB_TEAMS"))
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .filter(|s| !s.is_empty())
                .collect(),
            crew_members: std::env::var("RD_CREW_MEMBERS")
                .or_else(|_| std::env::var("CREW_MEMBERS"))
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            anthropic_api_key: std::env::var("RD_ANTHROPIC_API_KEY")
                .ok()
                .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok()),
        })
    }
}
