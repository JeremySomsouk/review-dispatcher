# filter

**Filter PRs by multiple criteria: repository, author, size, age, or specific PR number.**

Powerful way to slice through your pending reviews and find exactly what you're looking for.

## When to Use

- "Show me large PRs in the frontend repo"
- "What old PRs from alice are still waiting?"
- Building scripts that act on specific PRs
- Batch lookup of multiple specific PRs

## Synopsis

```bash
review-dispatcher filter [OPTIONS]
```

## Options

| Flag | Description |
|------|-------------|
| `PR_NUMBER` | Filter to a specific PR number |
| `--pr-numbers <NUMBERS>` | PR number(s) to filter to (comma-separated) |
| `--repo <NAME>` | Repository contains this text (partial match) |
| `--author <NAME>` | Author contains this text (partial match) |
| `--min-size <LINES>` | Minimum total lines changed |
| `--max-size <LINES>` | Maximum total lines changed |
| `--min-age <DAYS>` | PR is at least this many days old |
| `--max-age <DAYS>` | PR is at most this many days old |
| `-s, --since-days <DAYS>` | Only show PRs from the last N days |
| `--drafts-only` | Show only draft PRs |
| `--no-drafts` | Hide draft PRs |
| `-P, --priority` | Show priority scores |
| `--json` | Output as JSON |

Note: You can also use the global `--pr <NUMBER>` flag to target a specific PR.

## Examples

```bash
# Filter to specific PR
review-dispatcher filter 123

# All frontend PRs
review-dispatcher filter --repo frontend

# Big PRs only (over 500 lines)
review-dispatcher filter --min-size 500

# Small, quick PRs (under 50 lines)
review-dispatcher filter --max-size 50

# Old PRs that need attention
review-dispatcher filter --min-age 7 --priority

# Combine filters: old, large, non-draft PRs from backend
review-dispatcher filter --repo backend --min-age 3 --min-size 200 --no-drafts

# Find PRs by a specific author
review-dispatcher filter --author alice

# Recent PRs only (last 7 days)
review-dispatcher filter --since-days 7

# Combine filters: recent, large, non-draft PRs from backend
review-dispatcher filter --repo backend --since-days 7 --min-size 200 --no-drafts

# Batch lookup multiple PRs (parallel fetch)
review-dispatcher filter --pr-numbers 123,456,789

# Batch lookup with priority scores
review-dispatcher filter --pr-numbers 123,456,789 --priority

# JSON for scripting
review-dispatcher filter --repo api --min-size 100 --json | jq '.[].pr_number'

# Batch lookup and get JSON for scripting
review-dispatcher filter --pr-numbers 123,456,789 --json | jq '.[].url'
```

## Tips

- All filters are ANDed together (PR number AND repo AND author AND size...)
- Partial match on repo/author names (case-insensitive)
- Snoozed PRs are automatically hidden (use `--pr` to bypass snooze filter)
- When using `--pr-numbers`, other filters are bypassed and PRs are fetched directly in parallel
- Great for building automation scripts
