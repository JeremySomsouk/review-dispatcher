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
| `PR_NUMBER` | Filter to specific PR (shorthand for `--pr`) | - |
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
review-dispatcher stats

# Stats as JSON for scripting
review-dispatcher stats --json

# Stats for a specific repo only
review-dispatcher stats --repo frontend

# Stats filtered by author
review-dispatcher stats --author alice

# Stats for a specific PR
review-dispatcher stats 4821
review-dispatcher stats --pr 4821

# Stats with priority breakdown (includes most urgent PR highlight)
review-dispatcher stats --priority

# Stats for PRs created in the last 7 days only
review-dispatcher stats --since-days 7
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
