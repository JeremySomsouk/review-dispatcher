# filter

**Filter PRs by multiple criteria: repository, author, size, age.**

Powerful way to slice through your pending reviews and find exactly what you're looking for.

## When to Use

- "Show me large PRs in the frontend repo"
- "What old PRs from alice are still waiting?"
- Building scripts that act on specific PRs

## Synopsis

```bash
review-dispatcher filter [OPTIONS]
```

## Options

| Flag | Description |
|------|-------------|
| `--repo <NAME>` | Repository contains this text (partial match) |
| `--author <NAME>` | Author contains this text (partial match) |
| `--min-size <LINES>` | Minimum total lines changed |
| `--max-size <LINES>` | Maximum total lines changed |
| `--min-age <DAYS>` | PR is at least this many days old |
| `--max-age <DAYS>` | PR is at most this many days old |
| `--drafts-only` | Show only draft PRs |
| `--no-drafts` | Hide draft PRs |
| `-P, --priority` | Show priority scores |
| `--json` | Output as JSON |

## Examples

```bash
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

# JSON for scripting
review-dispatcher filter --repo api --min-size 100 --json | jq '.[].pr_number'
```

## Tips

- All filters are ANDed together (repo AND author AND size...)
- Partial match on repo/author names (case-insensitive)
- Great for building automation scripts
