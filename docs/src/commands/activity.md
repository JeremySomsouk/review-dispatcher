# activity

**Show your recent review activity — PRs you reviewed in the last N days.**

Track your review throughput and see what you've been up to.

## When to Use

- Weekly check: "How much did I review this week?"
- Performance reviews: "Show my review history"
- Team reporting: "Here's my review output"

## Synopsis

```bash
review-dispatcher activity [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-d, --days <DAYS>` | Number of days to look back | `7` |
| `--repo <REPO>` | Filter by repository name (partial match, case-insensitive) | all |
| `--author <AUTHOR>` | Filter by PR author username (partial match, case-insensitive) | all |
| `-p, --pr <PR>` | Show activity for specific PR (shorthand for global `--pr`) | all |
| `--json` | Output as JSON | `false` |

## Examples

```bash
# Last week's reviews
review-dispatcher activity

# Last 30 days
review-dispatcher activity --days 30

# Filter by repository
review-dispatcher activity --repo myrepo

# Filter by author
review-dispatcher activity --author johndoe

# Combine filters
review-dispatcher activity --days 14 --repo api --author alice

# Activity for specific PR
review-dispatcher activity --pr 123

# JSON output for scripting
review-dispatcher activity --json
```
