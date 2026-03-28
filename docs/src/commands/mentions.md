# mentions

**Show GitHub notifications where you were mentioned or directly involved.**

Stay on top of conversations without opening GitHub.

## When to Use

- Morning check: "Any mentions while I was offline?"
- Follow-up: "What threads am I in?"
- Check for activity on a specific PR

## Synopsis

```bash
review-dispatcher mentions [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-u, --unread` | Only show unread notifications | `false` |
| `-n, --limit <NUM>` | Limit results shown | `20` |
| `-p, --pr <NUM>` | Filter to specific PR number | all |
| `-s, --since-days <DAYS>` | Only show notifications from the last N days | all |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | all |
| `--author <PATTERN>` | Filter by repo pattern (partial match, case-insensitive) | all |
| `-P, --priority` | Show priority scores (1-5 stars based on age) | `false` |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# Show all notifications
review-dispatcher mentions

# Show only unread notifications
review-dispatcher mentions --unread

# Check notifications for a specific PR
review-dispatcher mentions --pr 123

# Combine with unread filter
review-dispatcher mentions --pr 123 --unread

# Only show notifications from the last 3 days
review-dispatcher mentions --since-days 3

# Filter by repository and time window
review-dispatcher mentions --repo myorg --since-days 7

# Filter by repo pattern (author alias for consistency)
review-dispatcher mentions --author myorg

# Show priority scores for urgent notifications
review-dispatcher mentions --priority

# Combine filters with priority
review-dispatcher mentions --repo myorg --priority --since-days 3

# JSON output for scripting
review-dispatcher mentions --json
```
