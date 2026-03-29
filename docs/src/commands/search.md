# search

**Search pending reviews by title keyword, optionally filtered by repo or author.**

Find specific PRs without scrolling through the full list.

## When to Use

- Remembering: "Was there a PR about auth?"
- Filtering: "Find all security-related PRs from a specific author"

## Synopsis

```bash
prctrl search [OPTIONS] [PR_NUMBER] <KEYWORD>
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `<KEYWORD>` | Keyword to search for in PR titles | Required |
| `--pr-numbers <NUMS>` | Search within specific PR numbers (comma-separated) | - |
| `PR_NUMBER` | Target a specific PR by number (shorthand for `--pr`) | None |
| `-s, --since-days <DAYS>` | Only show PRs from the last N days | all |
| `-p, --pr <NUM>` | Target a specific PR by number (bypasses search filters and snooze exclusion) | None |
| `--repo` | Filter by repository name (partial match, case-insensitive) | None |
| `--author` | Filter by author username (partial match, case-insensitive) | None |
| `--sort-by <FIELD>` | Sort results by: `priority`, `age`, `size`, or `title` | `priority` |
| `-P, --priority` | Show priority scores for each PR | `false` |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# Basic search (sorted by priority by default)
prctrl search auth

# Search within a specific repo
prctrl search auth --repo myservice

# Search for recent PRs only (last 7 days)
prctrl search auth --since-days 7

# Search for a PR by a specific author
prctrl search feature --author johndoe

# Sort by age (oldest first) instead of priority
prctrl search fix --sort-by age

# Sort by size (largest first)
prctrl search refactor --sort-by size

# Sort alphabetically by title
prctrl search update --sort-by title

# Combine filters with priority display
prctrl search fix --repo api --author alice --priority

# Target a specific PR (positional or --pr flag)
prctrl search anything 1234
prctrl search anything --pr 1234

# JSON output for scripting
prctrl search auth --json
```
