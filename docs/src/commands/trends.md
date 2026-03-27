# trends

**Analyze your review trends over time — velocity, week-over-week changes, top authors.**

See how your review patterns evolve. How fast are you reviewing? Is your backlog growing or shrinking? Who's sending the most PRs your way?

## When to Use

- End-of-sprint check: "Am I keeping up with reviews?"
- Personal metrics: track your review velocity over time
- Team health: spot if backlog is growing

## Synopsis

```bash
review-dispatcher trends [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-d, --days <NUM>` | Number of days to look back | `30` |
| `-n, --limit <NUM>` | Number of top authors/repos to show | `10` |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# Standard 30-day trend analysis
review-dispatcher trends

# Quick 7-day snapshot
review-dispatcher trends --days 7

# Focus on top 5 authors/repos
review-dispatcher trends --limit 5

# JSON for dashboards/scripts
review-dispatcher trends --json | jq '.avg_per_day'
```

## Output Example

```
📈 Review Trends — last 30 days

  📊 Summary
     Total reviews:       47
     Daily average:      1.6 PRs/day
     Lines reviewed:     +2840 / -1230
     Avg PR size:        +60 / -26

  📅 Week-over-Week
     📈 This week: 12   Previous: 9   Change: +33.3%

  📈 Daily Activity (last 14 days)
     03-21  ████████░░░░░░░░░░░░░  8
     03-22  ██████████████░░░░░░░  12
     ...

  👥 Top Authors (by PR count)
     alice      14
     bob        11
     carol       9

  📁 Top Repositories
     frontend    18
     backend    15
     shared      8
```

## How It Works

Trends reads the processed review files saved in your output directory (default: `./reviews/`). Each review file contains metadata about when it was reviewed, who authored it, and how large it was — trends aggregates this data to surface patterns.

The **week-over-week** comparison tells you if your review throughput is increasing or decreasing. The **sparkline chart** gives you a visual feel for daily rhythm — are you batch-reviewing on certain days?

## Tips

- Requires review files from `delegate` command (run `review-dispatcher delegate` first to generate them)
- Use `--days 7` for a tight weekly view, `--days 90` for quarterly context
- Pipe to `jq` for integration with monitoring dashboards
