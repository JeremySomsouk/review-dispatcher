# Review Dispatcher

Review Dispatcher is a terminal-native CLI tool that monitors GitHub PRs, sends you smart macOS notifications, and helps you review smarter.

## Features

- Smart Notifications - Native macOS alerts with crab branding
- AI Triage - Delegate PR review to Claude Code
- 30+ Commands - From listing PRs to viewing CI status
- Blazing Fast - Built in Rust

## Quick Start

```bash
# Install
cargo install --git https://github.com/JeremySomsouk/review-dispatcher

# Configure
cp .env.example .env
# Edit .env with your GitHub token and org

# List pending reviews
review-dispatcher list

# Monitor in background
review-dispatcher monitor
```

## Configuration

### Environment Variables

```bash
# Required
GITHUB_TOKEN=ghp_xxxxxxxxxxxx  # GitHub personal access token
GITHUB_ORG=my-org              # GitHub organization
GITHUB_REPOS=repo1,repo2       # Comma-separated repos
GITHUB_USERNAME=my-username    # Your GitHub username

# Optional
GITHUB_TEAMS=team1,team2       # Teams to filter by
CREW_MEMBERS=user1,user2       # Team members for crew mode
```

### Global Flags

- `-o, --output-dir` - Output folder (default: ./reviews)
- `-i, --instruction-path` - Custom instruction file
- `--open-terminal` - Open terminal in output dir
- `-m, --include-mine` - Include PRs you authored
- `-d, --include-drafts` - Include draft PRs
- `-c, --crew` - Only show crew member PRs
- `-p, --pr` - Target specific PR number
- `--exclude-prefix` - Exclude PR titles (comma-separated)

## Commands

### Core Commands

- `list` - List all PRs waiting for your review
- `delegate` - Ask Claude to triage each pending review
- `mine` - List your own open PRs
- `stats` - Show review statistics
- `team-summary` - Show team review summary

### PR Actions

- `assign` - Assign yourself as a reviewer
- `approve` - Approve a PR
- `claim` - Claim PRs for review
- `comment` - Post a comment
- `review` - Fetch and display PR diff

### Filtering and Search

- `search` - Search by title keyword
- `filter` - Filter by repo/author/size/age
- `top` - Show highest priority PRs
- `quick` - Show quick-win PRs (small, non-draft)
- `catchup` - Show oldest, ignored PRs
- `age` - Categorize PRs by age brackets

### Monitoring

- `monitor` - Monitor for new PRs + notifications
- `monitor-stop` - Stop the monitor process
- `monitor-status` - Check if monitor is running

### Information

- `diff` - Show diff/stats for a PR
- `files` - Show changed files
- `ci` - Show CI/CD pipeline status
- `conflicts` - Show PRs with merge conflicts
- `labels` - Show PR labels
- `activity` - Show your recent review activity
- `mentions` - Show GitHub notifications
- `summary` - One-line pending review summary
- `health` - Show GitHub API status

### Utilities

- `browse` - Open PRs in browser
- `snooze` - Temporarily hide PRs
- `report` - Generate weekly review report
- `clean` - Remove past review files

## License

MIT - Jeremy Somsouk
