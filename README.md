# 🦀 Review Dispatcher

> **Stop drowning in PR notifications. Let AI triage your reviews.**

Review Dispatcher is a terminal-native tool that monitors GitHub PRs, sends you smart macOS notifications, and delegates review triage to Claude — so you can focus on what matters.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![macOS](https://img.shields.io/badge/macOS-native-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

## ⚡ What It Does

```
┌─────────────────────────────────────────────────────────────┐
│  🔔 PR #4821: feat: add CSV export                        │
│  Review Dispatcher                                         │
│  👤 alice • +120 lines • opened 2 days ago                 │
│                                                             │
│  ✅ Chrome opens automatically → PR ready for review       │
└─────────────────────────────────────────────────────────────┘
```

**Monitor** your team's PRs in the background, get **notified** when new ones arrive, and **delegate** triage to Claude with one command.

## 🚀 Quick Start

```bash
# Install
cargo install --git https://github.com/JeremySomsouk/review-dispatcher

# Configure
cp .env.example .env
# Edit .env with your GitHub token and org

# Monitor for new PRs
review-dispatcher monitor

# List pending reviews
review-dispatcher list

# Let Claude triage everything
review-dispatcher delegate
```

**That's it.** No web UI. No Slack. No browser tabs.

## 🎯 Features

### 🔔 Smart Notifications
- Native macOS notifications with **🦀 crab emoji** branding
- Auto-opens PR in Chrome — click-free workflow
- Configurable polling interval (default: 5 minutes)
- `review-dispatcher monitor --notify` for background monitoring

### 🤖 AI Triage with Claude
```
┌────────────────────────────────────────────────────────
│ 🤖 Claude on: feat: add CSV export  #4821
│
│ This PR adds CSV export with validation.
│ Key areas: data privacy, performance.
│
│ Recommendation: [REVIEW NOW] — security-critical changes
└────────────────────────────────────────────────────────
```

Delegate to Claude Code for instant PR summaries and review recommendations.

### 📊 PR Management
- List all pending reviews with metadata
- Filter by team, author, or repository
- Crew mode for team-wide visibility
- Auto-excludes chore/deps PRs

### ⌨️ Interactive Mode
```
🔔 New PR detected: #123 - Fix login bug

🎯 Quick Actions:
  [d] Delegate to Claude    [o] Open in browser
  [i] Open in IntelliJ     [s] Skip
```

## 📁 Example Workflow

```bash
# 1. See what's waiting
review-dispatcher list

# Output:
# 🔍 3 pending review(s)
#   [1] feat: add CSV export  #4821 (frontend)
#   [2] fix: flaky CI        #312 (backend)
#   [3] chore: bump deps     #891 (excluded)

# 2. Let AI triage the important ones
review-dispatcher delegate

# 3. Monitor in background while you work
review-dispatcher monitor &
```

## ⚙️ Configuration

```env
# .env
RD_GITHUB_TOKEN=ghp_xxxxxxxxxxxxx
RD_GITHUB_USERNAME=your-username
RD_GITHUB_ORG=your-org
RD_GITHUB_REPOS=frontend,backend,api
RD_GITHUB_TEAMS=platform,backend
RD_CREW_MEMBERS=alice,bob,charlie
```

### Custom Review Instructions

Create an `instruction.md` file to customize Claude's review criteria:

```bash
# Project-specific (./instruction.md)
cp instruction.md.example instruction.md

# Or global (~/.review-dispatcher/instruction.md)
mkdir -p ~/.review-dispatcher
cp instruction.md.example ~/.review-dispatcher/instruction.md
```

## 🛠️ Installation

### From Source
```bash
git clone git@github.com:JeremySomsouk/review-dispatcher.git
cd review-dispatcher
cargo install --path .
```

### Requirements
- Rust 1.70+
- GitHub personal access token (fine-grained, PR read access)
- [Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code) (for `delegate` command)
- macOS (for notification features)

## 📖 CLI Reference

| Command | Description |
|---------|-------------|
| `review-dispatcher list` | List all pending reviews |
| `review-dispatcher mine` | List your own open PRs |
| `review-dispatcher delegate` | Triage with Claude |
| `review-dispatcher delegate 4821` | Triage specific PR |
| `review-dispatcher monitor` | Background monitoring + notifications |
| `review-dispatcher monitor --interactive` | Interactive quick actions |
| `review-dispatcher clean` | Remove past review files |

### Shell Aliases

```bash
alias reviews="review-dispatcher list"
alias triage="review-dispatcher delegate"
alias monitor="review-dispatcher monitor"
```

## 🔮 Roadmap

- [ ] Slack integration
- [ ] PR prioritization scoring
- [ ] Review metrics dashboard
- [ ] Windows/Linux notification support

## 🤝 Contributing

Open issues or PRs welcome — bug fixes, features, docs, or performance improvements.

## License

MIT
