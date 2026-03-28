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
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
    },
    /// Ask Claude to triage each pending review
    Delegate {
        /// Output as JSON (useful for scripting)
        #[arg(long)]
        json: bool,
        /// Preview delegation without executing (show what would be delegated)
        #[arg(long, short = 'n')]
        dry_run: bool,
        /// Show priority scores for each PR (1-5 stars based on age and size)
        #[arg(long, short = 'P')]
        priority: bool,
        /// Only delegate PRs created since this many days ago
        #[arg(long, short = 's')]
        since_days: Option<u32>,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Delegate all matching PRs at once (without prompting)
        #[arg(long, short = 'a')]
        all: bool,
    },
    /// List your own open PRs (draft or not)
    Mine {
        /// Output as JSON (useful for scripting)
        #[arg(long)]
        json: bool,
        /// Show priority score (1-5 stars) based on age and size
        #[arg(long, short = 'P')]
        priority: bool,
        /// Only show PRs created since this many days ago
        #[arg(long, short = 's')]
        since_days: Option<u32>,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
    },
    /// Show review statistics (pending count, avg wait time, breakdown by repo)
    Stats {
        /// PR number to show stats for (shorthand for --pr)
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// Output as JSON for scripting
        #[arg(long)]
        json: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Show priority scores for each PR (1-5 stars based on age and size)
        #[arg(long, short = 'P')]
        priority: bool,
        /// Only show PRs created since this many days ago
        #[arg(long, short = 's')]
        since_days: Option<u32>,
    },
    /// Show team review summary (how many PRs each crew member has waiting)
    TeamSummary {
        /// PR number to show team summary for (shorthand for --pr)
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// Output as JSON for scripting
        #[arg(long)]
        json: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Show priority scores for each PR (1-5 stars based on age and size)
        #[arg(long, short = 'P')]
        priority: bool,
        /// Only show PRs created since this many days ago
        #[arg(long, short = 's')]
        since_days: Option<u32>,
    },
    /// Show review workload distribution across team members (load balance analysis)
    Load {
        /// Minimum number of PRs to be considered "loaded" (default: 3)
        #[arg(long, short)]
        threshold: Option<u32>,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Output as JSON for scripting
        #[arg(long)]
        json: bool,
    },
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
        /// Output as JSON
        #[arg(long)]
        json: bool,
        /// Show priority score for the PR (1-5 stars based on age and size)
        #[arg(long, short = 'P')]
        priority: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
    },
    /// Show full PR information including description, reviewers, and metadata
    Info {
        /// PR number to show info for
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
        /// Show priority score for the PR (1-5 stars based on age and size)
        #[arg(long, short = 'P')]
        priority: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
    },
    /// Show the chronological timeline of events on a PR (reviews, comments, labels, CI, etc.)
    Timeline {
        /// PR number to show timeline for
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
    },
    /// Assign yourself as a reviewer on a PR
    Assign {
        /// Assign yourself to all pending reviews at once
        #[arg(long, short = 'a')]
        all: bool,
        /// PR number(s) to assign to (comma-separated)
        #[arg(value_name = "PR_NUMBERS")]
        pr_numbers: Option<String>,
        /// PR number to assign yourself to (shorthand for --pr)
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Remove yourself as a reviewer from a PR
    Unassign {
        /// Unassign yourself from all pending reviews at once
        #[arg(long, short = 'a')]
        all: bool,
        /// PR number(s) to unassign from (comma-separated)
        #[arg(value_name = "PR_NUMBERS")]
        pr_numbers: Option<String>,
        /// PR number to unassign yourself from (shorthand for --pr)
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Post a comment on a PR directly from the CLI
    Comment {
        /// Comment on all pending reviews at once
        #[arg(long, short = 'a')]
        all: bool,
        /// PR number(s) to comment on (comma-separated)
        #[arg(value_name = "PR_NUMBERS")]
        pr_numbers: Option<String>,
        /// PR number to comment on
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// Comment text (supports markdown)
        #[arg(long, short = 't', value_name = "TEXT")]
        text: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Approve a PR directly from the CLI
    Approve {
        /// Approve all pending reviews at once
        #[arg(long, short = 'a')]
        all: bool,
        /// PR number(s) to approve (comma-separated)
        #[arg(value_name = "PR_NUMBERS")]
        pr_numbers: Option<String>,
        /// PR number to approve
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// Approve with a custom comment
        #[arg(long, short = 'm')]
        message: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Claim multiple PRs for review at once
    Claim {
        /// Claim all pending reviews at once
        #[arg(long, short = 'a')]
        all: bool,
        /// PR number(s) to claim (comma-separated)
        #[arg(value_name = "PR_NUMBERS")]
        pr_numbers: Option<String>,
        /// Preview what would be claimed without taking action
        #[arg(long, short = 'n')]
        dry_run: bool,
        /// Show priority scores for each PR (1-5 stars based on age and size)
        #[arg(long, short = 'P')]
        priority: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Open one or more PRs in your browser
    Browse {
        /// PR number to open (shorthand for --pr)
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// PR number(s) to open (comma-separated)
        #[arg(long, short = 'n')]
        pr_numbers: Option<String>,
        /// Open specific PR (shorthand for --pr)
        #[arg(long, short = 'p')]
        pr: Option<u64>,
        /// Open all pending reviews
        #[arg(long, short = 'a')]
        all: bool,
        /// Preview which PRs would be opened without actually opening them
        #[arg(long)]
        dry_run: bool,
        /// Output URLs as JSON (useful for scripting)
        #[arg(long)]
        json: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
    },
    /// Show changed files for one or more PRs
    Files {
        /// PR number to show files for (shorthand for --pr)
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// PR number(s) to show files for (comma-separated)
        #[arg(long, short = 'n')]
        pr_numbers: Option<String>,
        /// Show files for specific PR (shorthand for --pr)
        #[arg(long, short = 'p')]
        pr: Option<u64>,
        /// Show files for all pending reviews
        #[arg(long, short = 'a')]
        all: bool,
        /// Show priority scores for each PR (1-5 stars based on age and size)
        #[arg(long, short = 'P')]
        priority: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
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
        /// Only show PRs created since this many days ago
        #[arg(long, short = 's')]
        since_days: Option<u32>,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Sort results by: priority, age, size, or title (default: priority)
        #[arg(long, value_name = "FIELD", default_value = "priority")]
        sort_by: Option<String>,
        /// Show priority scores for search results
        #[arg(long, short = 'P')]
        priority: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Filter pending reviews by various criteria
    Filter {
        /// PR number to filter to (shorthand for --pr)
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
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
        /// Only show PRs created since this many days ago
        #[arg(long, short = 's')]
        since_days: Option<u32>,
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
        /// PR number to check CI for (shorthand for --pr)
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// Show CI status for specific PR (shorthand for --pr)
        #[arg(long, short = 'p')]
        pr: Option<u64>,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show which pending PRs have merge conflicts
    Conflicts {
        /// Only show PRs with conflicts (hide clean PRs)
        #[arg(long, short = 'c')]
        only_conflicts: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show labels for one or more PRs
    Labels {
        /// PR number to show labels for (shorthand for --pr)
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// PR number(s) to show labels for (comma-separated)
        #[arg(long, short = 'n')]
        pr_numbers: Option<String>,
        /// Show labels for specific PR (shorthand for --pr)
        #[arg(long, short = 'p')]
        pr: Option<u64>,
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
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
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
        /// Filter to specific PR number
        #[arg(long, short = 'p')]
        pr: Option<u64>,
        /// Only show notifications from the last N days
        #[arg(long, short = 's')]
        since_days: Option<u32>,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author/repo pattern (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Show priority scores for each notification (1-5 stars based on age and repo count)
        #[arg(long, short = 'P')]
        priority: bool,
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
    /// Analyze which PRs demand your immediate attention based on multiple urgency factors
    Attention {
        /// Only show PRs with attention score >= threshold (1-10, default: 5)
        #[arg(long, short = 't')]
        threshold: Option<u8>,
        /// Show detailed breakdown of why each PR scored high
        #[arg(long, short = 'd')]
        detailed: bool,
        /// Limit the number of results shown (default: 10)
        #[arg(long, short = 'n')]
        limit: Option<usize>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show the ONE PR you should focus on right now — the most urgent by priority score
    Focus {
        /// Open the focused PR in your browser instead of printing details
        #[arg(long, short = 'o')]
        open: bool,
        /// Output as JSON (includes full PR details)
        #[arg(long)]
        json: bool,
        /// Show priority score for the focused PR (1-5 stars based on age and size)
        #[arg(long, short = 'P')]
        priority: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
    },
    /// Show GitHub API health status and rate limits
    Health {
        /// Output as JSON
        #[arg(long)]
        json: bool,
        /// Show actionable recommendations based on rate limits
        #[arg(long, short = 's')]
        suggest: bool,
    },
    /// Show your highest priority pending PRs based on age, size, and urgency
    Top {
        /// Limit the number of results shown (default: 10)
        #[arg(long, short = 'n')]
        limit: Option<usize>,
        /// Minimum priority score threshold (1-5, default: 3)
        #[arg(long, short = 's')]
        min_score: Option<u8>,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
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
        /// Show priority scores for quick wins (1-5 stars based on age and size)
        #[arg(long, short = 'P')]
        priority: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
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
        /// Show priority scores for each PR (1-5 stars based on age and size)
        #[arg(long, short = 'P')]
        priority: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Show all neglected PRs without limit (no truncation)
        #[arg(long, short = 'l')]
        all: bool,
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
        /// Show priority scores for each PR (1-5 stars based on age and size)
        #[arg(long, short = 'P')]
        priority: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Categorize pending PRs by size (XS/S/M/L/XL) with statistics and visual breakdown
    Size {
        /// Show only PRs of specific size(s) - XS, S, M, L, XL (comma-separated)
        #[arg(long, short = 'f')]
        filter_size: Option<String>,
        /// Group output by size bucket instead of flat list
        #[arg(long, short = 'g', default_value_t = false)]
        grouped: bool,
        /// Show priority scores for each PR
        #[arg(long, short = 'P')]
        priority: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Fetch and display a PR diff in the terminal with syntax highlighting
    Review {
        /// PR number to review (shorthand for --pr)
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// Show diff for specific PR (shorthand for --pr)
        #[arg(long, short = 'p')]
        pr: Option<u64>,
        /// Show diffs for all pending reviews
        #[arg(long, short = 'a')]
        all: bool,
        /// Number of context lines around changes (default: 3)
        #[arg(long, short = 'C', default_value_t = 3)]
        context: u8,
        /// Output diff to file instead of terminal
        #[arg(long, short = 'o')]
        output_file: Option<PathBuf>,
        /// Language hint for syntax highlighting (auto-detected if not specified)
        #[arg(long, short = 'l')]
        language: Option<String>,
        /// Show priority score for each PR (1-5 stars based on age and size)
        #[arg(long, short = 'P')]
        priority: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Generate a shareable weekly digest (markdown, perfect for Slack/Teams)
    Digest {
        /// Number of days to include (default: 7)
        #[arg(long, short = 'd', default_value_t = 7)]
        days: u32,
        /// Output as raw Markdown (no preamble)
        #[arg(long)]
        raw: bool,
    },
    /// Analyze review trends over time (velocity, avg time to review, top reviewers)
    Trends {
        /// Number of days to analyze (default: 30)
        #[arg(long, short = 'd', default_value_t = 30)]
        days: u32,
        /// Number of top authors/reviewers to show (default: 10)
        #[arg(long, short = 'n')]
        limit: Option<usize>,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Output as JSON for scripting
        #[arg(long)]
        json: bool,
    },
    /// Analyze how quickly PRs get reviewed (avg time-to-first-review, bottleneck detection)
    ReviewVelocity {
        /// Number of days to look back (default: 30)
        #[arg(long, short = 'd', default_value_t = 30)]
        days: u32,
        /// Show bottleneck analysis (which repos/authors take longest)
        #[arg(long, short = 'b')]
        bottlenecks: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Output as JSON for scripting
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
        /// Output as JSON (useful for scripting)
        #[arg(long)]
        json: bool,
        /// Show priority scores for listed snoozed PRs
        #[arg(long, short = 'P')]
        priority: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
    },
    /// Send a follow-up reminder to authors of stale PRs to get their attention
    Chase {
        /// PR number to chase (shorthand for --pr)
        #[arg(value_name = "PR_NUMBER")]
        pr_number: Option<u64>,
        /// Minimum age in days to chase (default: 7)
        #[arg(long, short = 'a', default_value_t = 7)]
        min_age: u32,
        /// Send actual comments instead of just previewing
        #[arg(long, short = 's')]
        send: bool,
        /// Custom message template (use {author} and {title} as placeholders)
        #[arg(long, short = 'm')]
        message: Option<String>,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Estimate review time for pending PRs based on size and complexity
    ReviewTime {
        /// PR number(s) to estimate (comma-separated)
        #[arg(value_name = "PR_NUMBERS")]
        pr_numbers: Option<String>,
        /// Show review time for all pending reviews
        #[arg(long, short = 'a')]
        all: bool,
        /// Group output by time category (lightning/quick/moderate/substantial/lengthy)
        #[arg(long, short = 'g')]
        grouped: bool,
        /// Show priority scores for each PR (1-5 stars based on age and size)
        #[arg(long, short = 'P')]
        priority: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Export pending reviews to CSV or Markdown table format
    Export {
        /// Export format: csv or markdown (default: csv)
        #[arg(long, short = 'f', value_name = "FORMAT")]
        format: Option<String>,
        /// Output file path (stdout if not specified)
        #[arg(long, short = 'o')]
        output: Option<PathBuf>,
        /// Include only these columns (comma-separated): repo,number,title,author,size,age,draft,url
        #[arg(long, short = 'c')]
        columns: Option<String>,
        /// Export all pending reviews (not just current session)
        #[arg(long, short = 'a')]
        all: bool,
        /// Output as JSON (useful for scripting)
        #[arg(long)]
        json: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
    },
    /// Search and filter your review history from processed files
    History {
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Filter by review state (APPROVED, CHANGES_REQUESTED, COMMENTED)
        #[arg(long, short = 's')]
        state: Option<String>,
        /// Number of days to look back (default: 30)
        #[arg(long, short = 'd', default_value_t = 30)]
        days: u32,
        /// Limit the number of results shown (default: 50)
        #[arg(long, short = 'n')]
        limit: Option<usize>,
        /// Output as JSON for scripting
        #[arg(long)]
        json: bool,
    },
    /// Show PRs that are ready to merge (approved, CI passing, no conflicts)
    Ready {
        /// Show PRs ready for specific repo
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Compare two PRs side-by-side to help decide which to review first
    Compare {
        /// First PR number to compare (in format "repo#123" or just "123" for first repo)
        #[arg(value_name = "PR1")]
        pr1: String,
        /// Second PR number to compare (in format "repo#123" or just "123" for first repo)
        #[arg(value_name = "PR2")]
        pr2: String,
        /// Show detailed comparison including file-level breakdown
        #[arg(long, short = 'd')]
        detailed: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Watch PRs for status changes (CI passing, approved, conflicts resolved, etc.)
    Follow {
        /// Follow action: add, list, remove, clear, or status
        #[command(subcommand)]
        action: FollowAction,
        /// PR number(s) to follow (comma-separated, for add/remove)
        #[arg(value_name = "PR_NUMBERS")]
        pr_numbers: Option<String>,
        /// Output as JSON for scripting
        #[arg(long)]
        json: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
    },
    /// Show PRs that are blocked from merging (CI failures, conflicts, or other issues)
    Blocked {
        /// Filter to specific repository
        #[arg(long, short = 'r')]
        repo: Option<String>,
        /// Only show PRs with failing CI
        #[arg(long, short = 'c')]
        ci_only: bool,
        /// Only show PRs with merge conflicts
        #[arg(long, short = 'm')]
        conflicts_only: bool,
        /// Limit the number of results shown (default: 20)
        #[arg(long, short = 'n')]
        limit: Option<usize>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Send emoji reactions to PR authors to get their attention (non-intrusive nudge)
    Ping {
        /// Reaction emoji to send: eyes, rocket, heart, +1 (default: eyes)
        #[arg(long, short = 'e', value_name = "EMOJI", default_value = "eyes")]
        emoji: String,
        /// PR number(s) to ping (comma-separated)
        #[arg(value_name = "PR_NUMBERS")]
        pr_numbers: Option<String>,
        /// Ping all pending reviews
        #[arg(long, short = 'a')]
        all: bool,
        /// Send actual pings instead of previewing
        #[arg(long, short = 's')]
        send: bool,
        /// Filter by repository name (partial match, case-insensitive)
        #[arg(long)]
        repo: Option<String>,
        /// Filter by author username (partial match, case-insensitive)
        #[arg(long)]
        author: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum FollowAction {
    /// Add PR(s) to your follow list
    Add,
    /// List currently followed PRs and their last known status
    List,
    /// Remove PR(s) from your follow list
    Remove,
    /// Clear all followed PRs
    Clear,
    /// Check for status changes since last check
    Status,
}

#[derive(Subcommand, Debug)]
pub enum SnoozeAction {
    /// Add PR(s) to the snooze list
    Add,
    /// List currently snoozed PRs
    List,
    /// Show detailed info about snoozed PRs (author, age, lines changed, priority)
    Review,
    /// Remove PR(s) from the snooze list
    Remove,
    /// Clear all snoozed PRs
    Clear,
    /// Remove expired snooze entries (PRs whose snooze time has passed)
    Expire,
    /// Extend the snooze duration for already-snoozed PRs
    Extend,
}
