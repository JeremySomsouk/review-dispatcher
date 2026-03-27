# mentions

**Show GitHub notifications where you were mentioned or directly involved.**

Stay on top of conversations without opening GitHub.

## When to Use

- Morning check: "Any mentions while I was offline?"
- Follow-up: "What threads am I in?"

## Synopsis

```bash
review-dispatcher mentions [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-u, --unread` | Only show unread notifications | `false` |
| `-n, --limit <NUM>` | Limit results shown | `20` |

## Examples

```bash
review-dispatcher mentions
review-dispatcher mentions --unread
```
