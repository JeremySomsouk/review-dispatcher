# summary

**Show a quick one-line summary of pending reviews — total count, oldest age, lines changed, and urgency breakdown.**

The fastest way to get an at-a-glance view of your review queue without scrolling through a full list.

## When to Use

- **Slack check**: "How backed up am I?"
- **Before a 1:1**: "Quick status on pending reviews"
- **Shell prompt**: Drop it in your terminal prompt or status bar

## Synopsis

```bash
review-dispatcher summary [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--json` | Output as JSON (great for scripting/status bars) | `false` |

## Examples

```bash
# Quick human-readable summary
review-dispatcher summary

# JSON for scripting (e.g., terminal status bar)
review-dispatcher summary --json

# Parse in a script
review-dispatcher summary --json | jq '.total'
```

## Output Example

```
📊 12 PRs pending | oldest: 5 days | +1,847 / -623 lines | 2 drafts
```

## JSON Output

When `--json` is used, output includes:

```json
{
  "total": 12,
  "total_additions": 1847,
  "total_deletions": 623,
  "oldest_age_days": 5,
  "draft_count": 2,
  "repos": {
    "frontend": 5,
    "backend": 4,
    "shared": 3
  }
}
```

## Tips

- Pipe to `jq` for quick metrics in scripts
- Combine with `watch` for a live dashboard: `watch -n 300 review-dispatcher summary`
- Great for embedding in shell prompts or status bars
