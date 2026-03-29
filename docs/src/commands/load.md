# load

**Show review workload distribution across team members.**

Analyzes how review requests are distributed across your team, identifying who is overloaded and who has capacity. Helps team leads balance review load and make informed delegation decisions.

## When to Use

- Sprint planning: understand who can take on more reviews
- Identify bottlenecks: find team members with too many pending PRs
- Balance workload: redistribute reviews before standup
- Health checks: ensure no one is overwhelmed

## Synopsis

```bash
review-dispatcher load [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--threshold, -t <N>` | Minimum PRs to be considered "loaded" | `3` |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | |
| `--since-days, -s <N>` | Only show PRs created since this many days ago | |
| `-p, --pr <NUMBER>` | Target a specific PR by number | - |
| `--pr-numbers <NUMS>` | Target specific PRs by number (comma-separated) | - |
| `--priority, -P` | Show priority scores for each PR (1-5 stars based on age and size) | `false` |
| `--json` | Output as JSON for scripting | `false` |

**Note:** The global `--pr` flag (`-p`) also works with this command for consistency with other commands.

## Examples

```bash
# Show workload distribution with default threshold (3 PRs)
review-dispatcher load

# Set custom overload threshold (5 PRs)
review-dispatcher load --threshold 5

# Filter to a specific repository
review-dispatcher load --repo myservice

# Filter to a specific author
review-dispatcher load --author sarah_dev

# Only show recent PRs (last 7 days) to focus on fresh requests
review-dispatcher load --since-days 7

# Combine filters
review-dispatcher load --repo myservice --author sarah_dev --threshold 4

# Show priority breakdown to identify urgent PRs
review-dispatcher load --priority

# JSON output for automation/dashboards
review-dispatcher load --json
```

## Output

```
⚖️  Review Load Distribution
──────────────────────────────────────────────────
  Total pending PRs: 24 | Team members: 8 | Overload threshold: 3 PRs

  Workload bar (max 6 PRs):
  ████████████████                                  🟢 3  
  ████████████████████████████              🟢 6  
  ████████████████████████████████  🟢 7  

  Author                     PRs     +add      -del    Avg Age    Status
  ──────────────────────────────────────────────────────────────────────
  sarah_dev                     7     +892      -124     4d       🔴 OVERLOADED
  bob_eng                       6     +445       -89     2d       🔴 OVERLOADED
  alice_ops                     3     +120       -34     1d       🔴 OVERLOADED
  charlie_dev                   2     +234       -45     3d       🟢 OK
  dan_backend                   2     +567       -78     5d       🟢 OK
  eve_frontend                  2     +189       -23     1d       🟢 OK
  frank_dev                      1     +234       -45     2d       🟢 OK
  grace_dev                      1     +98        -12     1d       🟢 OK
  ──────────────────────────────────────────────────────────────────────
  Summary: 5 healthy | 3 overloaded

  💡 Recommendations:
  • sarah_dev has the most pending PRs (7), consider reassigning some
  • Average load: 3.0 PRs per member
  • Consider delegating to: frank_dev, grace_dev
```

## Output Fields

| Field | Description |
|-------|-------------|
| **Author** | GitHub username who created the PR |
| **PRs** | Number of pending review requests |
| **+add** | Total additions across all their PRs |
| **-del** | Total deletions across all their PRs |
| **Avg Age** | Average age of their pending PRs |
| **Status** | 🟢 OK or 🔴 OVERLOADED based on threshold |
| **repos** | List of repositories their PRs are from |

## Status Rules

- **Overloaded** 🔴: PR count >= threshold (default: 3)
- **OK** 🟢: PR count < threshold

## Recommendations

The command provides actionable recommendations:

1. **Top overloaded member**: Who has the most pending PRs
2. **Average load**: Mean PRs per team member
3. **Underloaded members**: Who has capacity to take more

## Tips

- Use `--threshold 5` in larger teams to reduce noise
- Combine with `review-dispatcher team-summary` for broader team view
- Use `--json` output to build team dashboards
- Run before sprint planning to balance review load
- Use `--repo` to focus on specific repository workload distribution
- Use `--author` to see load breakdown for specific team members
- Use `--since-days` to focus on recent PRs only (e.g., `--since-days 7` for last week's requests)

## Related Commands

- [`team-summary`](./team-summary.md) — Overview of team activity
- [`stats`](./stats.md) — Review statistics
- [`filter`](./filter.md) — Filter by author, repo, size
- [`assign`](./assign.md) — Reassign PRs to balance load
