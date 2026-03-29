# stats

**Show review statistics: pending count, average wait time, and breakdowns by repository and author.**

Gives you a high-level overview of your review queue at a glance.

## When to Use

- Morning overview: "How bad is it?"
- Sprint planning: "What's the review load?"

## Synopsis

```bash
prctrl stats [OPTIONS]
prctrl stat [OPTIONS]      # shorthand alias
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-a, --all` | Show stats for all matching PRs without interactive selection | `false` |
| `--pr-numbers <NUMS>` | Show stats for specific PR numbers (comma-separated) | - |
| `PR_NUMBER` | Filter to specific PR (shorthand for `--pr`) | - |
| `-n, --dry-run` | Preview which PRs would be included without showing stats | `false` |
| `--json` | Output as JSON | `false` |
| `-p, --pr <NUMBER>` | Filter to a specific PR number | - |
| `--repo <NAME>` | Filter by repository (partial match, case-insensitive) | - |
| `--author <NAME>` | Filter by author (partial match, case-insensitive) | - |
| `-P, --priority` | Show priority breakdown and highlight most urgent PR | `false` |
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | - |

**Note:** The global `--pr` flag (`-p`) also works with this command for consistency with other commands.

## Examples

```bash
# See all review statistics
prctrl stats

# Stats as JSON for scripting
prctrl stats --json

# Stats for a specific repo only
prctrl stats --repo frontend

# Stats filtered by author
prctrl stats --author alice

# Stats for a specific PR
prctrl stats 4821
prctrl stats --pr 4821

# Stats with priority breakdown (includes most urgent PR highlight)
prctrl stats --priority

# Stats for PRs created in the last 7 days only
prctrl stats --since-days 7

# Stats for all matching PRs without interactive selection
prctrl stats --all

# Preview which PRs would be included in stats
prctrl stats --dry-run

# Combine --all with filters
prctrl stats --all --repo frontend --priority
```

## Output

### Normal Mode
- Total pending reviews count
- Total lines changed (+additions / -deletions)
- Average time waiting
- Oldest PR info
- Breakdown by repository (sorted by count)
- Breakdown by author (with visual bar chart)

### Priority Mode (`--priority`)
When `--priority` is enabled, `stats` also shows:

**🚨 Most Urgent** — A highlighted callout showing the single PR that demands your immediate attention, including:
- PR title and number with priority stars
- Author, size, age, and repository
- Direct link to the PR

**Priority Breakdown** — PRs grouped by score (1-5 stars):
- Shows count of PRs at each priority level
- Oldest PR age within each group
- Total lines changed per group

This helps you understand both the overall queue health and the most critical item needing action.

## Dry-Run Mode

When using `--dry-run`, the command shows a preview of which PRs would be included in the stats without actually computing or displaying the statistics. This is useful for:
- Checking which PRs match your filters before running full stats
- Verifying filter criteria are correct
- Quickly listing matching PRs
