# activity

**Show your recent review activity — PRs you reviewed in the last N days.**

Track your review throughput and see what you've been up to.

## When to Use

- Weekly check: "How much did I review this week?"
- Performance reviews: "Show my review history"
- Team reporting: "Here's my review output"

## Synopsis

```bash
review-dispatcher activity [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-d, --days <DAYS>` | Number of days to look back | `7` |
| `--json` | Output as JSON | `false` |

## Examples

```bash
# Last week's reviews
review-dispatcher activity

# Last 30 days
review-dispatcher activity --days 30
```
