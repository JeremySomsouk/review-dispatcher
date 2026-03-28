# top

**Show your highest priority pending PRs — sorted by age, size, and urgency.**

The PRs that need your attention most.

## When to Use

- Priority planning: "What matters most?"
- Neglect check: "What have I been ignoring?"

## Synopsis

```bash
review-dispatcher top [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-n, --limit <NUM>` | Maximum results shown | `10` |
| `-s, --min-score <NUM>` | Minimum priority score (1-5) | `3` |
| `--repo <PATTERN>` | Filter by repository (partial match, case-insensitive) | |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | |

## Examples

```bash
# Show top 10 priority PRs
review-dispatcher top

# Show top 5 priority PRs
review-dispatcher top --limit 5

# Show only critical PRs (score >= 4)
review-dispatcher top --min-score 4

# Show top PRs from a specific repo
review-dispatcher top --repo my-service

# Show top PRs from a specific author
review-dispatcher top --author johndoe

# Combine filters
review-dispatcher top --repo my-service --author johndoe --min-score 4
```
