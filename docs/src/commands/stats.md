# stats

**Show review statistics: pending count, average wait time, and breakdowns by repository and author.**

Gives you a high-level overview of your review queue at a glance.

## When to Use

- Morning overview: "How bad is it?"
- Sprint planning: "What's the review load?"

## Synopsis

```bash
review-dispatcher stats [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--json` | Output as JSON | `false` |

## Examples

```bash
review-dispatcher stats
review-dispatcher stats --json
```
