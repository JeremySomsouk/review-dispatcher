use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "review-dispatcher")]
pub struct Cli {
    /// Folder where review files will be written (default: ./reviews)
    #[arg(long, short, global = true)]
    pub output_dir: Option<PathBuf>,

    /// Path to custom instruction file (default: searches in multiple locations)
    #[arg(long, global = true)]
    pub instruction_path: Option<PathBuf>,

    /// Open a new terminal tab in output_dir after running
    #[arg(long, global = true)]
    pub open_terminal: bool,

    /// Include PRs you authored
    #[arg(long, short = 'm', global = true)]
    pub include_mine: bool,

    /// Include draft PRs
    #[arg(long, short = 'd', global = true)]
    pub include_drafts: bool,

    /// Only show open PRs from crew members (CREW_MEMBERS in .env)
    #[arg(long, short = 'c', global = true)]
    pub crew: bool,

    /// Exclude PRs whose title matches these prefixes (comma-separated, default: "chore(deps)")
    #[arg(long, global = true, value_delimiter = ',', default_value = "chore(deps)")]
    pub exclude_prefix: Vec<String>,

    /// Target a specific PR by number (bypasses review-request filters)
    #[arg(long, short = 'p', global = true)]
    pub pr: Option<u64>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all PRs waiting for your review
    List,
    /// Ask Claude to triage each pending review
    Delegate {
        /// PR number (shorthand for --pr)
        #[arg(value_name = "PR_NUMBER")]
        pr_positional: Option<u64>,
    },
    /// List your own open PRs (draft or not)
    Mine,
    /// Remove all past review files from the output directory
    Clean,
    /// Monitor for new PRs and send macOS notifications
    Monitor {
        /// Polling interval in seconds (default: 300)
        #[arg(long, short, default_value_t = 300)]
        interval: u64,
        /// Send macOS notifications for new PRs
        #[arg(long, short, default_value_t = true)]
        notify: bool,
        /// Automatically open PRs in Chrome when notifications appear
        #[arg(long, default_value_t = true)]
        auto_open: bool,
        
        /// Disable automatic opening of PRs in Chrome
        #[arg(long, overrides_with = "auto_open")]
        no_auto_open: bool,
        /// Interactive mode - prompt for actions on new PRs
        #[arg(long)]
        interactive: bool,
    },
    /// Stop the running monitor process
    MonitorStop,
    /// Check if monitor process is running
    MonitorStatus,
}
