# PRCtrl

> **Terminal-native GitHub PR management. Stay on top of code reviews without leaving your terminal.**

PRCtrl helps engineering teams manage PR reviews efficiently. Monitor incoming PRs, get smart notifications, and integrate with AI for instant triage — all from the command line.

[![crates.io](https://img.shields.io/crates/v/prctrl.svg)](https://crates.io/crates/prctrl)
[![crates.io](https://img.shields.io/crates/d/prctrl.svg)](https://crates.io/crates/prctrl)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![macOS](https://img.shields.io/badge/macOS-native-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)
[![Docs](https://img.shields.io/badge/Documentation-Online-blue.svg)](https://jeremysomsouk.github.io/prctrl/)

## What It Does

```
$ prctrl list

  3 pending reviews

  [1] feat: add CSV export        #4821  alice    backend    +120/-30   2 days
  [2] fix: authentication bug     #3156  bob      frontend   +45/-12    1 day
  [3] refactor: API client        #3102  charlie  backend   +280/-90   4 days
```

**Core features:**
- **List & filter** PRs across repos, teams, and authors
- **Monitor** for new PRs with native macOS notifications
- **Delegate** triage to Claude for instant recommendations
- **Track** review history and team metrics
- **Stack detection** automatically identifies PRs that build on each other

## Quick Start

```bash
# Install
cargo install prctrl

# Configure
prctrl config init

# List pending reviews
prctrl list

# Monitor for new PRs (background)
prctrl monitor
```

## Installation

### From Source
```bash
cargo install --git https://github.com/JeremySomsouk/prctrl
```

### Requirements
- Rust 1.70+
- GitHub personal access token (read access to PRs)
- macOS (for native notifications)

## Configuration

Run the interactive setup to configure PRCtrl:

```bash
prctrl config init
```

This creates a config file at `~/.prctrl/config.toml` with your GitHub settings.

**Example configuration:**

```toml
# ~/.prctrl/config.toml

[github]
token = "ghp_xxxxxxxxxxxxxxxxxxxx"
username = "john_doe"
org = "my-company"
repos = ["frontend", "backend", "mobile"]
teams = ["platform", "backend"]  # optional
```

**Getting a GitHub Token:**

1. Go to GitHub → Settings → Developer Settings → Personal Access Tokens
2. Generate New Token (Classic)
3. Select scopes: `repo`, `read:user`, `notifications`
4. Copy the token and add it to your config

**Optional: Configure Crew Members**

The `crew` feature filters PRs to show only those from your team members:

```toml
[github]
# ... required fields above ...
crew_members = ["alice", "bob", "carol"]

**Alternative: Environment Variables**

Instead of a config file, you can use environment variables:

| Variable | Description |
|----------|-------------|
| `PRCTRL_GITHUB_TOKEN` | GitHub personal access token |
| `PRCTRL_GITHUB_USERNAME` | Your GitHub username |
| `PRCTRL_GITHUB_ORG` | GitHub organization name |
| `PRCTRL_GITHUB_REPOS` | Repos to monitor (comma-separated) |
| `PRCTRL_GITHUB_TEAMS` | Teams to filter (optional) |
| `PRCTRL_ANTHROPIC_API_KEY` | For Claude integration (optional) |

## CLI Reference

| Command | Description |
|---------|-------------|
| `prctrl list` | List pending reviews |
| `prctrl mine` | Your own open PRs |
| `prctrl top` | Highest priority PRs |
| `prctrl delegate [pr]` | AI triage with Claude |
| `prctrl chat` | Interactive chat with Claude |
| `prctrl monitor` | Background monitoring |
| `prctrl approve <pr>` | Quick approve |
| `prctrl chase <pr>` | Follow up stale PRs |
| `prctrl stats` | Team review metrics |

See `prctrl --help` for full command list.

## Workflow Example

```bash
# Morning: See what needs attention
prctrl list --priority

# Quick: Approve trivial PRs
prctrl approve 4821

# Deep work: Delegate triage to AI
prctrl delegate

# End of day: Check team stats
prctrl team-summary
```

## Integrations

### Claude Integration
```bash
# Get AI-powered review summary
prctrl delegate 4821

# Uses your existing Claude Code CLI
# Configure instructions in ~/.prctrl/instruction.md
```

### Interactive Chat
```bash
# Chat with Claude about your PRs
prctrl chat

# Chat about a specific PR
prctrl chat --pr 4821
```

The `chat` command launches an interactive Claude Code session with context about PRCtrl commands. Ask questions about PRs, get recommendations on what to review, or learn how to use PRCtrl features.


## Stack Detection

PRCtrl automatically detects **stacked PRs** — PRs that build on each other through branch relationships. This helps you identify dependent PRs that need to be reviewed in sequence.

### How it works

Stack detection analyzes branch relationships:
- PR A targets branch `feature`
- PR B targets branch `feature-2` (or builds on `feature`)
- This creates a stack: `feature` → `feature-2`

### Usage

```bash
# Show stacked PRs in your own PRs (automatic)
prctrl mine

# Show stacked PRs in pending reviews (opt-in)
prctrl list --show-stacks
```

### Example Output

```
┌─ Stack on `main` (3 PRs)

🔵 #123 - Add new feature
  └─ @feature
    https://github.com/owner/repo/pull/123

  #124 - Implement API endpoint
  └─ @feature-2
    https://github.com/owner/repo/pull/124

  #125 - Add tests
  └─ @feature-3
    https://github.com/owner/repo/pull/125
```

The blue dot (🔵) indicates the base PR of each stack.

## Documentation

- [Full Documentation](https://jeremysomsouk.github.io/prctrl/) — User guide and command reference
- [CONTRIBUTING.md](./CONTRIBUTING.md) — How to contribute to PRCtrl
- [TROUBLESHOOTING.md](./TROUBLESHOOTING.md) — Common issues and solutions

Issues and PRs welcome!

## License

MIT
