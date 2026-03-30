# PRCtrl

**Stop drowning in PR notifications. Let AI triage your reviews.**

PRCtrl is your terminal-native PR companion. It watches your GitHub PRs, notifies you when action is needed, and helps you review smarter with AI-powered triage.

```
┌─────────────────────────────────────────────────────────────┐
│  🔔 PR #4821: feat: add CSV export                        │
│  PRCtrl                                         │
│  👤 alice • +120 lines • opened 2 days ago                 │
│                                                             │
│  ✅ Chrome opens automatically → PR ready for review       │
└─────────────────────────────────────────────────────────────┘
```

## Why PRCtrl?

- **No more tab switching** — Everything in your terminal
- **AI-powered triage** — Let Claude analyze PRs for you
- **Smart notifications** — macOS native alerts, auto-opens PRs
- **30+ commands** — From quick glances to deep dive analysis

## Installation

```bash
cargo install --git https://github.com/JeremySomsouk/prctrl
```

## Quick Start

```bash
# 1. Configure your GitHub credentials
prctrl config init

# 2. See what needs your attention
prctrl list

# 3. Let AI triage the important ones
prctrl delegate

# 4. Monitor in background (get notified of new PRs)
prctrl monitor &
```

## Your First Review Workflow

```bash
# See all PRs waiting for you
prctrl list

# Output:
# 🔍 3 pending review(s)
#   [1] feat: add CSV export        #4821 (frontend)  👤 alice +120 - 2 days
#   [2] fix: flaky CI              #312 (backend)    👤 bob   +45  - 4 days
#   [3] chore: update deps         #891 (deps)       👤 carol +890 - 1 day

# Want to focus on important stuff only?
prctrl quick --max-lines 200

# Search for a specific PR?
prctrl search "security"

# Let Claude decide what's important?
prctrl delegate
```

## Common Workflows

### Morning Check (5 seconds)

```bash
prctrl summary
# Output: "📊 12 PRs pending • Oldest: 5 days • 3 urgent • 2 quick wins"
```

### Before a Meeting

```bash
# Grab the essentials
prctrl top --limit 5

# Check if any have failing CI
prctrl ci --failed-only
```

### Deep Work Session

```bash
# Start monitoring (you'll be notified of new PRs)
prctrl monitor --interval 300 &

# When you get a notification...
prctrl review --pr 4821  # Read the diff without leaving terminal
prctrl diff --pr 4821     # See stats

# Approve directly from CLI
prctrl approve --pr 4821 -m "LGTM! Good tests."
```

### End of Week

```bash
# Generate a report of what you reviewed
prctrl report --days 7

# See your activity
prctrl activity --days 7
```

## Configuration

### Interactive Setup

```bash
prctrl config init
```

This creates a configuration file at `~/.prctrl/config.toml`:

```toml
[github]
token = "ghp_xxxxxxxxxxxxxxxxxxxx"
username = "john_doe"
org = "my-company"
repos = ["frontend", "backend"]
teams = ["platform", "backend"]  # optional
```

### Environment Variables (Alternative)

Instead of a config file, you can use environment variables:

```bash
# Required
PRCTRL_GITHUB_TOKEN=ghp_xxxxxxxxxxxx
PRCTRL_GITHUB_USERNAME=john_doe
PRCTRL_GITHUB_ORG=my-company
PRCTRL_GITHUB_REPOS=frontend,backend

# Optional
PRCTRL_GITHUB_TEAMS=platform,backend
PRCTRL_CREW_MEMBERS=alice,bob,carol
PRCTRL_ANTHROPIC_API_KEY=sk-ant-xxxxxxxxxxxx
```

### Getting a GitHub Token

1. Go to GitHub → Settings → Developer Settings → Personal Access Tokens
2. Generate New Token (Classic)
3. Select scopes: `repo`, `read:user`, `notifications`
4. Add the token to your config or environment

## All Commands

### Core Commands

| Command | Description |
|---------|-------------|
| [list](./commands/list.md) | List all PRs waiting for your review |
| [delegate](./commands/delegate.md) | Ask Claude to triage each PR |
| [mine](./commands/mine.md) | List your own open PRs |
| [stats](./commands/stats.md) | See your review statistics |
| [team-summary](./commands/team-summary.md) | Team-wide pending PR view |

### Taking Action

| Command | Description |
|---------|-------------|
| [assign](./commands/assign.md) | Assign yourself to a PR |
| [approve](./commands/approve.md) | Approve a PR |
| [claim](./commands/claim.md) | Claim PRs for review |
| [comment](./commands/comment.md) | Post a comment |
| [review](./commands/review.md) | Full diff review in terminal |

### Finding PRs

| Command | Description |
|---------|-------------|
| [search](./commands/search.md) | Search by keyword in title |
| [filter](./commands/filter.md) | Filter by repo/author/size/age |
| [top](./commands/top.md) | Highest priority PRs |
| [quick](./commands/quick.md) | Small PRs you can knock out fast |
| [catchup](./commands/catchup.md) | Oldest, most-ignored PRs |
| [age](./commands/age.md) | PRs grouped by age bracket |

### Information

| Command | Description |
|---------|-------------|
| [diff](./commands/diff.md) | Stats and changes for a PR |
| [files](./commands/files.md) | List changed files |
| [ci](./commands/ci.md) | CI/CD pipeline status |
| [conflicts](./commands/conflicts.md) | PRs with merge conflicts |
| [labels](./commands/labels.md) | PR labels |
| [summary](./commands/summary.md) | One-line overview |
| [health](./commands/health.md) | GitHub API status & rate limits |

### Background Monitoring

| Command | Description |
|---------|-------------|
| [monitor](./commands/monitor.md) | Watch for new PRs + notify |
| [monitor-stop](./commands/monitor-stop.md) | Stop monitoring |
| [monitor-status](./commands/monitor-status.md) | Check if running |

### Utilities

| Command | Description |
|---------|-------------|
| [browse](./commands/browse.md) | Open PRs in browser |
| [snooze](./commands/snooze.md) | Hide PRs temporarily |
| [report](./commands/report.md) | Weekly review report |
| [clean](./commands/clean.md) | Clear cached review files |
| [activity](./commands/activity.md) | Your recent review history |
| [mentions](./commands/mentions.md) | GitHub notifications |

## Global Flags

These work with most commands:

| Flag | Description |
|------|-------------|
| `-o, --output-dir <DIR>` | Output folder (default: `./reviews`) |
| `-p, --pr <NUMBER>` | Target specific PR |
| `-m, --include-mine` | Include your own PRs |
| `-d, --include-drafts` | Include draft PRs |
| `-c, --crew` | Only show team member PRs |
| `--json` | Output as JSON for scripting |

## License

MIT — Jeremy Somsouk
