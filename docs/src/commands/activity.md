# activity

**Show your recent review activity — PRs you reviewed in the last N days.**

Track your review throughput and see what you've been up to.

## When to Use

- Weekly check: "How much did I review this week?"
- Performance reviews: "Show my review history"
- Team reporting: "Here's my review output"

## Synopsis

```bash
prctrl activity [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-d, --days <DAYS>` | Number of days to look back | `7` |
| `--repo <REPO>` | Filter by repository name (partial match, case-insensitive) | all |
| `--author <AUTHOR>` | Filter by PR author username (partial match, case-insensitive) | all |
| `-p, --pr <PR>` | Show activity for specific PR (shorthand for global `--pr`) | all |
| `-P, --priority` | Show priority scores for each PR (1-5 stars based on age and size) | `false` |
| `--json` | Output as JSON | `false` |
| `-s, --since-days <DAYS>` | Only show PRs reviewed since this many days ago | all |

## Examples

```bash
# Last week's reviews
prctrl activity

# Last 30 days
prctrl activity --days 30

# Filter by repository
prctrl activity --repo myrepo

# Filter by author
prctrl activity --author johndoe

# Combine filters
prctrl activity --days 14 --repo api --author alice

# Activity for specific PR
prctrl activity --pr 123

# Show priority scores for reviewed PRs
prctrl activity --priority

# JSON output for scripting
prctrl activity --json

# Only show PRs reviewed in the last 7 days (even if --days is 30)
prctrl activity --days 30 --since-days 7
```
