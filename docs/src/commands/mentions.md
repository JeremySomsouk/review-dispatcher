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

# JSON output for scripting
review-dispatcher mentions --json
```
