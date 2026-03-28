# catchup

**Show PRs you should catch up on — oldest, longest-ignored, sorted by neglect.**

Perfect for after vacation or when returning to a busy sprint.

## When to Use

- Back from vacation: "What did I miss?"
- Weekly review: "What have I been neglecting?"

## Synopsis

```bash
review-dispatcher catchup [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to show catchup for (shorthand for `--pr`) | |
| `-a, --min-age <DAYS>` | Minimum age in days to be considered "catchup" | `3` |
| `-n, --limit <NUM>` | Limit the number of results shown | `10` |
| `-P, --priority` | Show priority scores (1-5 stars based on age and size) | `false` |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | |
| `-l, --all` | Show all neglected PRs without limit | `false` |
| `--json` | Output as JSON | `false` |

## Examples

```bash
# Show PRs older than 3 days
review-dispatcher catchup

# Focus on week-old+ PRs
review-dispatcher catchup --min-age 7

# Show priority scores for each PR
review-dispatcher catchup --priority

# Combine with limit for more results
review-dispatcher catchup --min-age 7 --limit 20

# Filter by repository
review-dispatcher catchup --repo myservice

# Filter by author
review-dispatcher catchup --author johndoe

# Show ALL neglected PRs without truncation
review-dispatcher catchup --all

# Combine filters for targeted catchup
review-dispatcher catchup --min-age 7 --repo api --author johndoe --priority

# Target a specific PR (bypasses --min-age filter)
review-dispatcher catchup --pr 123
review-dispatcher catchup 123
```
