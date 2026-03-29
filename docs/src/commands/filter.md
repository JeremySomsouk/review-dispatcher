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
prctrl filter [OPTIONS]
```

## Options

| Flag | Description |
|------|-------------|
| `PR_NUMBER` | Filter to a specific PR number (shorthand for `--pr`) |
| `-a, --all` | Show all filtered results without prompting for selection |
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
# Filter to specific PR (via positional arg)
prctrl filter 123

# Filter to specific PR (via --pr flag)
prctrl filter --pr 123

# All frontend PRs
prctrl filter --repo frontend

# Big PRs only (over 500 lines)
prctrl filter --min-size 500

# Small, quick PRs (under 50 lines)
prctrl filter --max-size 50

# Old PRs that need attention
prctrl filter --min-age 7 --priority

# Combine filters: old, large, non-draft PRs from backend
prctrl filter --repo backend --min-age 3 --min-size 200 --no-drafts

# Find PRs by a specific author
prctrl filter --author alice

# Recent PRs only (last 7 days)
prctrl filter --since-days 7

# Combine filters: recent, large, non-draft PRs from backend
prctrl filter --repo backend --since-days 7 --min-size 200 --no-drafts

# Batch lookup multiple PRs (parallel fetch)
prctrl filter --pr-numbers 123,456,789

# Batch lookup with priority scores
prctrl filter --pr-numbers 123,456,789 --priority

# JSON for scripting
prctrl filter --repo api --min-size 100 --json | jq '.[].pr_number'

# Batch lookup and get JSON for scripting
prctrl filter --pr-numbers 123,456,789 --json | jq '.[].url'

# Show all large PRs without interactive selection
prctrl filter --min-size 500 --all

# Find old PRs from a specific author and show all at once
prctrl filter --author alice --min-age 7 --all --priority
```

## Tips

- All filters are ANDed together (PR number AND repo AND author AND size...)
- Partial match on repo/author names (case-insensitive)
- Snoozed PRs are automatically hidden (use `--pr` to bypass snooze filter)
- When using `--pr-numbers`, other filters are bypassed and PRs are fetched directly in parallel
- Use `--all` flag to skip interactive selection and show all filtered results at once
- Great for building automation scripts
