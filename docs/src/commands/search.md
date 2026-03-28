# search

**Search pending reviews by title keyword, optionally filtered by repo or author.**

Find specific PRs without scrolling through the full list.

## When to Use

- Remembering: "Was there a PR about auth?"
- Filtering: "Find all security-related PRs from a specific author"

## Synopsis

```bash
review-dispatcher search [OPTIONS] <KEYWORD>
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `<KEYWORD>` | Keyword to search for in PR titles | Required |
| `-s, --since-days <DAYS>` | Only show PRs from the last N days | all |
| `-p, --pr <NUM>` | Target a specific PR by number (bypasses search filters and snooze exclusion) | None |
| `--repo` | Filter by repository name (partial match, case-insensitive) | None |
| `--author` | Filter by author username (partial match, case-insensitive) | None |
| `-P, --priority` | Sort results by priority (highest first) and show scores | `false` |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# Basic search
review-dispatcher search auth

# Search within a specific repo
review-dispatcher search auth --repo myservice

# Search for recent PRs only (last 7 days)
review-dispatcher search auth --since-days 7

# Search for a PR by a specific author
review-dispatcher search feature --author johndoe

# Combine filters
review-dispatcher search fix --repo api --author alice --priority

# Target a specific PR (even if snoozed)
review-dispatcher search anything --pr 1234

# JSON output for scripting
review-dispatcher search auth --json
```
