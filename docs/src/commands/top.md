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
| `-m, --min-score <NUM>` | Minimum priority score (1-5) | `3` |

## Examples

```bash
review-dispatcher top
review-dispatcher top --limit 5
```
