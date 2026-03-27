# quick

**Find small, non-draft PRs you can review in 5 minutes.**

When you have 2 minutes before a meeting or just want to clear easy wins, `quick` shows you the PRs that are small enough to knock out fast.

## When to Use

- Quick mental break between tasks
- Morning energy check
- Need a small win to start the day

## Synopsis

```bash
review-dispatcher quick [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-l, --max-lines <NUM>` | Max total lines to be considered "quick" | `200` |
| `-n, --limit <NUM>` | Max results to show | `10` |
| `--json` | Output as JSON | `false` |

## Examples

```bash
# Default: PRs under 200 lines
review-dispatcher quick

# Even smaller PRs only (under 50 lines)
review-dispatcher quick --max-lines 50

# More results
review-dispatcher quick --limit 20

# Show as JSON
review-dispatcher quick --json
```

## Tips

- Great for code freeze periods when you want to stay productive without deep dives
- Combine with `approve` to quickly clear small PRs
