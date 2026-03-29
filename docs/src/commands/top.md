# top

**Show your highest priority pending PRs — sorted by age, size, and urgency.**

The PRs that need your attention most.

## When to Use

- Priority planning: "What matters most?"
- Neglect check: "What have I been ignoring?"

## Synopsis

```bash
prctrl top [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-n, --limit <NUM>` | Maximum results shown | `10` |
| `--min-score <NUM>` | Minimum priority score (1-5) | `3` |
| `-P, --priority` | Show priority stars (1-5) for each PR | |
| `--repo <PATTERN>` | Filter by repository (partial match, case-insensitive) | |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | |
| `--since-days <DAYS>` | Only show PRs created within the last N days | |

## Examples

```bash
# Show top 10 priority PRs
prctrl top

# Show top 5 priority PRs
prctrl top --limit 5

# Show only critical PRs (score >= 4)
prctrl top --min-score 4

# Show top PRs from a specific repo
prctrl top --repo my-service

# Show top PRs from a specific author
prctrl top --author johndoe

# Combine filters
prctrl top --repo my-service --author johndoe --min-score 4

# Show priority stars alongside scores
prctrl top --priority

# Show top PRs from the last 7 days only
prctrl top --since-days 7

# Show top PRs from the last 14 days in a specific repo
prctrl top --since-days 14 --repo my-service
```

## Notes

- Snoozed PRs are automatically excluded from results (consistent with `list`, `delegate`, `search`, etc.)
- Use `prctrl snooze add` to temporarily hide PRs from results

## Related Commands

- [`attention`](./attention.md) — Multi-factor urgency analysis
- [`focus`](./focus.md) — Show the single most urgent PR
- [`summary`](./summary.md) — Quick one-line overview
- [`snooze`](./snooze.md) — Temporarily hide PRs from results
