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
    /// Approve a PR directly from the CLI
    Approve {
        /// PR number to approve
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// Approve with a custom comment
        #[arg(long, short = 'm')]
        message: Option<String>,
    },
    /// Claim multiple PRs for review at once
    Claim {
        /// Claim all pending reviews at once
        #[arg(long, short = 'a')]
        all: bool,
        /// PR number(s) to claim (comma-separated)
        #[arg(value_name = "PR_NUMBERS")]
        pr_numbers: Option<String>,
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
    /// Generate a weekly review report from processed review files
    Report {
        /// Number of days to look back (default: 7)
        #[arg(long, default_value_t = 7)]
        days: u32,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Search pending reviews by title keyword
    Search {
        /// Keyword to search for in PR titles
        #[arg(value_name = "KEYWORD")]
        query: String,
        /// Show priority scores for search results
        #[arg(long, short = 'P')]
        priority: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
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
    /// Show CI/CD pipeline status for pending PRs (GitHub Actions, etc.)
    Ci {
        /// Only show PRs with failing checks
        #[arg(long, short = 'f')]
        failed_only: bool,
        /// Only show PRs with passing checks
        #[arg(long, short = 'p')]
        passing_only: bool,
        /// Show CI status for all pending reviews
        #[arg(long, short = 'a')]
        all: bool,
        /// PR number(s) to check (comma-separated)
        #[arg(value_name = "PR_NUMBERS")]
        pr_numbers: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show which pending PRs have merge conflicts
    Conflicts {
        /// Only show PRs with conflicts (hide clean PRs)
        #[arg(long, short = 'c')]
        only_conflicts: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show labels for one or more PRs
    Labels {
        /// PR number(s) to show labels for (comma-separated)
        #[arg(value_name = "PR_NUMBERS")]
        pr_numbers: Option<String>,
        /// Show labels for all pending reviews
        #[arg(long, short = 'a')]
        all: bool,
        /// Filter pending reviews by label name (partial match, case-insensitive)
        #[arg(long, short = 'l')]
        filter_by: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show your recent review activity (PRs you reviewed in the last N days)
    Activity {
        /// Number of days to look back (default: 7)
        #[arg(long, short = 'd', default_value_t = 7)]
        days: u32,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show GitHub notifications where you were mentioned or directly involved
    Mentions {
        /// Only show unread notifications
        #[arg(long, short = 'u')]
        unread_only: bool,
        /// Limit the number of results shown (default: 20)
        #[arg(long, short = 'n')]
        limit: Option<usize>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show a quick one-line summary of pending reviews (total, oldest age, lines, urgency breakdown)
    Summary {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Fetch and display a PR diff in the terminal with syntax highlighting
    Review {
        /// PR number to review
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// Number of context lines around changes (default: 3)
        #[arg(long, short = 'C', default_value_t = 3)]
        context: u8,
        /// Output diff to file instead of terminal
        #[arg(long)]
        output_file: Option<PathBuf>,
        /// Language hint for syntax highlighting (auto-detected if not specified)
        #[arg(long, short = 'l')]
        language: Option<String>,
    },
    /// Show your highest priority pending PRs based on age, size, and urgency
    Top {
        /// Limit the number of results shown (default: 10)
        #[arg(long, short = 'n')]
        limit: Option<usize>,
        /// Minimum priority score threshold (1-5, default: 3)
        #[arg(long, short = 's')]
        min_score: Option<u8>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show "quick win" PRs — small, non-draft PRs you can review quickly
    Quick {
        /// Maximum total lines for a "quick" PR (default: 200)
        #[arg(long, short = 'l')]
        max_lines: Option<u64>,
        /// Limit the number of results shown (default: 10)
        #[arg(long, short = 'n')]
        limit: Option<usize>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show PRs you should catch up on — oldest, longest-ignored, sorted by neglect
    Catchup {
        /// Minimum age in days to be considered "catchup" (default: 3)
        #[arg(long, short = 'a', default_value_t = 3)]
        min_age: u32,
        /// Limit the number of results shown (default: 10)
        #[arg(long, short = 'n')]
        limit: Option<usize>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Categorize pending PRs by age brackets (new/aging/stale/overdue) with visual buckets
    Age {
        /// Show only PRs newer than this many days
        #[arg(long, short = 'n')]
        min_days: Option<u32>,
        /// Show only PRs older than this many days
        #[arg(long, short = 'x')]
        older_than: Option<u32>,
        /// Group output by age bucket instead of flat list
        #[arg(long, short = 'g', default_value_t = false)]
        grouped: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Temporarily hide PRs from the pending list (snooze them)
    Snooze {
        /// Snooze action: add, list, remove, or clear
        #[command(subcommand)]
        action: SnoozeAction,
        /// PR number(s) to snooze (comma-separated)
        #[arg(value_name = "PR_NUMBERS")]
        pr_numbers: Option<String>,
        /// Snooze duration in days (default: 3)
        #[arg(long, short = 'd')]
        days: Option<u32>,
    },
}

#[derive(Subcommand, Debug)]
pub enum SnoozeAction {
    /// Add PR(s) to the snooze list
    Add,
    /// List currently snoozed PRs
    List,
    /// Remove PR(s) from the snooze list
    Remove,
    /// Clear all snoozed PRs
    Clear,
}
