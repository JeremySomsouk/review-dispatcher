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
| `--json` | Output as JSON | `false` |

## Examples

```bash
review-dispatcher catchup
review-dispatcher catchup --min-age 1
```
