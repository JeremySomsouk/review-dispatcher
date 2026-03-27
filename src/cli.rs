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
    List {
        /// Output as JSON (useful for scripting)
        #[arg(long)]
        json: bool,
        /// Only show PRs created since this many days ago
        #[arg(long, short = 's')]
        since_days: Option<u32>,
        /// Show priority score (1-5 stars) based on age and size
        #[arg(long, short = 'P')]
        priority: bool,
    },
    /// Ask Claude to triage each pending review
    Delegate {
        /// PR number (shorthand for --pr)
        #[arg(value_name = "PR_NUMBER")]
        pr_positional: Option<u64>,
    },
    /// List your own open PRs (draft or not)
    Mine,
    /// Show review statistics (pending count, avg wait time, breakdown by repo)
    Stats,
    /// Show team review summary (how many PRs each crew member has waiting)
    TeamSummary,
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
    /// Show diff/stats for a specific PR directly in terminal
    Diff {
        /// PR number to show diff for
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
    },
    /// Assign yourself as a reviewer on a PR
    Assign {
        /// PR number to assign yourself to
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
    },
    /// Open one or more PRs in your browser
    Browse {
        /// PR number(s) to open (comma-separated)
        #[arg(value_name = "PR_NUMBERS")]
        pr_numbers: Option<String>,
    },
    /// Show changed files for one or more PRs
    Files {
        /// PR number(s) to show files for (comma-separated)
        #[arg(value_name = "PR_NUMBERS")]
        pr_numbers: Option<String>,
        /// Show files for all pending reviews
        #[arg(long, short = 'a')]
        all: bool,
    },
    /// Filter pending reviews by various criteria
    Filter {
        /// Filter by repository name (partial match)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match)
        #[arg(long)]
        author: Option<String>,
        /// Minimum total lines changed (+additions -deletions)
        #[arg(long)]
        min_size: Option<u64>,
        /// Maximum total lines changed
        #[arg(long)]
        max_size: Option<u64>,
        /// Minimum age in days
        #[arg(long)]
        min_age: Option<u32>,
        /// Maximum age in days
        #[arg(long)]
        max_age: Option<u32>,
        /// Show only draft PRs
        #[arg(long)]
        drafts_only: bool,
        /// Show only non-draft PRs
        #[arg(long)]
        no_drafts: bool,
        /// Show priority scores for filtered results
        #[arg(long, short = 'P')]
        priority: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}
