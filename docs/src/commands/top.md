# top

**Show your highest priority PRs based on age, size, and neglect.**

Calculates a priority score (1-5 stars) based on:
- Days waiting (older = higher priority)
- Size (larger = higher priority)  
- Draft status (non-draft = higher priority)

## When to Use

- "What should I absolutely look at today?"
- Before a sprint planning
- After vacation

## Synopsis

```bash
review-dispatcher top [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-n, --limit <NUM>` | Max results | `10` |
| `-s, --min-score <NUM>` | Minimum score threshold (1-5) | `3` |
| `--json` | Output as JSON | `false` |

## Examples

```bash
# Top 10 priority PRs
review-dispatcher top

# Only the most critical (score 4+)
review-dispatcher top --min-score 4

# Get just the numbers for scripts
review-dispatcher top --min-score 3 --json | jq '.[].pr_number'
```

## Tips

- Start here if you're overwhelmed
- Use `--min-score 5` to find the absolute most urgent
