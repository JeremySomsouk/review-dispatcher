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
| `-a, --min-age <DAYS>` | Minimum age to be considered "catchup" | `3` |
| `-n, --limit <NUM>` | Limit results shown | `10` |
| `-P, --priority` | Show priority scores (1-5 stars based on age and size) | `false` |
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
```
