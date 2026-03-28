# review-velocity

**Analyze how quickly PRs get reviewed — average time-to-review, distribution breakdown, and bottleneck detection.**

See how fast (or slow) your team is at reviewing PRs. `review-velocity` calculates the time from PR creation to when you submitted your review, then breaks it down by time buckets and identifies which repositories or authors create the slowest reviews.

## When to Use

- End of sprint: "Are we reviewing fast enough?"
- Identify bottlenecks: "Which repos/authors take forever to review?"
- Set SLAs: "What a reasonable review time target?"
- Track improvement: "Did our new process help?"

## Synopsis

```bash
review-dispatcher review-velocity [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-d, --days <NUM>` | Number of days to look back | `30` |
| `-b, --bottlenecks` | Show bottleneck analysis by author/repo | `false` |
| `-P, --priority` | Show priority vs review time breakdown | `false` |
| `--repo <TEXT>` | Filter by repository name (partial match, case-insensitive) | |
| `--author <TEXT>` | Filter by author username (partial match, case-insensitive) | |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# Standard 30-day velocity analysis
review-dispatcher review-velocity

# Quick 7-day snapshot
review-dispatcher review-velocity --days 7

# With bottleneck analysis (see slowest repos/authors)
review-dispatcher review-velocity --bottlenecks

# With priority breakdown (correlate priority scores with review speed)
review-dispatcher review-velocity --priority

# Combine all analysis options
review-dispatcher review-velocity --bottlenecks --priority

# Filter by specific repository
review-dispatcher review-velocity --repo backend

# Filter by specific author
review-dispatcher review-velocity --author alice

# Combine filters with bottleneck analysis
review-dispatcher review-velocity --repo api --bottlenecks

# JSON for dashboards
review-dispatcher review-velocity --json | jq '.avg_hours_to_review'
```

## Output Example

```
⚡ Review Velocity — last 30 days
─────────────────────────────────────────────

  📊 Summary (23 PRs reviewed)
     Average time to review:  18.4 hours
     Median time to review:    12.0 hours
     Fastest review:           0.5 hours
     Slowest review:           72.0 hours

  ⏱️  Time Distribution
     < 4h:       4 (17.4%)  ▓▓▓▓░░░░░░░░░░░░░░
     4-24h:      8 (34.8%)  ▓▓▓▓▓▓▓░░░░░░░░░░
     1-3d:       7 (30.4%)  ▓▓▓▓▓▓▓░░░░░░░░░
     > 3d:       4 (17.4%)  ▓▓▓▓░░░░░░░░░░░░░░

  ⭐ Priority vs Review Time
     ⭐⭐⭐⭐⭐  5 PRs  28.3h avg  ████████████
     ⭐⭐⭐⭐    8 PRs  14.2h avg  ████████
     ⭐⭐⭐     10 PRs  8.5h avg   █████
     ⭐⭐        0 PRs   0.0h avg
     ⭐         0 PRs   0.0h avg

  🐢 Bottleneck Analysis — by Author
     (slowest average review time)
     alice     ██████████  36.2h avg  (5 PRs)
     bob       ████████    28.1h avg  (8 PRs)
     carol     ███████      22.4h avg  (4 PRs)

  🐢 Bottleneck Analysis — by Repository
     (slowest average review time)
     backend   ██████████  32.1h avg  (12 PRs)
     frontend  ███████      18.5h avg  (7 PRs)
     shared    ████        8.2h avg   (4 PRs)

  💡 Use `--bottlenecks` to see which repos/authors take longest
  💡 Use `--priority` to correlate priority with review speed
  💡 Use `--json` for machine-readable output
─────────────────────────────────────────────
```

## How It Works

Review velocity reads the processed review files saved in your output directory (default: `./reviews/`). Each file contains both `Created` and `Reviewed on` timestamps, allowing calculation of the review time in hours.

**Time buckets:**
- **< 4h** — Same day reviews (fast turnaround)
- **4-24h** — Next day reviews (typical)
- **1-3d** — Multi-day reviews (getting slow)
- **> 3d** — Stale reviews (bottleneck indicator)

**Bottleneck analysis** shows which authors or repositories have the highest average review times, helping identify process or knowledge gaps.

**Priority vs Review Time** (with `--priority` flag) shows how long high vs low priority PRs take to review. This helps answer questions like "Are we prioritizing urgent reviews?"

## Tips

- Requires review files from `delegate` command
- Use `--days 7` for weekly view, `--days 90` for quarterly context
- Track your velocity over time to see if process changes help
- Pair with `trends` for complete review analytics
- Use `--priority` to see if critical PRs are getting the attention they deserve
