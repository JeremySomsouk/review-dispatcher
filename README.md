# PRCtrl

> **Terminal-native GitHub PR management. Stay on top of code reviews without leaving your terminal.**

PRCtrl helps engineering teams manage PR reviews efficiently. Monitor incoming PRs, get smart notifications, and integrate with AI for instant triage — all from the command line.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![macOS](https://img.shields.io/badge/macOS-native-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

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
git clone https://github.com/JeremySomsouk/prctrl.git
cd prctrl
cargo install --path .
```

### Requirements
- Rust 1.70+
- GitHub personal access token (read access to PRs)
- macOS (for native notifications)

## Configuration

```bash
# Interactive setup
prctrl config init

# Or create ~/.prctrl/config.toml manually
[github]
token = "ghp_xxxxxxxxxxxx"
username = "your-username"
org = "your-org"
repos = ["frontend", "backend", "api"]
teams = ["platform", "backend"]

[notifications]
enabled = true
interval = 300  # seconds
```

## CLI Reference

| Command | Description |
|---------|-------------|
| `prctrl list` | List pending reviews |
| `prctrl mine` | Your own open PRs |
| `prctrl top` | Highest priority PRs |
| `prctrl delegate [pr]` | AI triage with Claude |
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

## Contributing

Issues and PRs welcome. Please read the docs in `docs/` before contributing.

## License

MIT
