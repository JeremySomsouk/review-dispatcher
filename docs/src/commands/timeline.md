# timeline

**Show the chronological timeline of events on a PR (reviews, comments, labels, CI, etc.).**

The `timeline` command displays a chronological history of all events on a PR — who requested reviews, when reviews were submitted, comments added, labels changed, branch updates, and more. It's like a "git log" for the PR's lifecycle.

## When to Use

- Understand the full history of a PR before diving into review
- See when and why review requests were made
- Track down who made specific comments or requested changes
- Identify bottlenecks in the PR review process
- See if a PR has been sitting idle or has active discussion

## Synopsis

```bash
prctrl timeline [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | Target a specific PR by number | Interactive selection |
| `--pr, -p` | Target a specific PR by number (global) | Interactive selection |
| `--pr-numbers` | Show timeline for multiple PRs (comma-separated) | Single PR |
| `--all, -a` | Show timeline for all pending reviews (no interactive selection) | `false` |
| `--dry-run, -n` | Preview which PRs would be shown without displaying timelines | `false` |
| `--json` | Output as JSON for scripting | `false` |
| `--repo` | Filter by repository name (partial match, case-insensitive) | All repos |
| `--author` | Filter by author username (partial match, case-insensitive) | All authors |
| `-P, --priority` | Show priority score for each PR (1-5 stars based on age and size) | `false` |
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | All ages |

## Examples

```bash
# Show timeline for a specific PR
prctrl timeline --pr 4821

# Show timelines for multiple PRs at once (fetched in parallel)
prctrl timeline --pr-numbers 4821,4822,4823

# Show timeline for all pending reviews (no interactive selection)
prctrl timeline --all

# Preview which PRs would be shown (dry run)
prctrl timeline --all --dry-run

# Interactive mode (select from pending reviews)
prctrl timeline

# JSON output for integration
prctrl timeline --pr 4821 --json

# Filter to a specific repo
prctrl timeline --repo frontend

# Filter to a specific author
prctrl timeline --author sarah_dev

# Combine filters
prctrl timeline --repo myorg --author reviewer1

# Show with priority scores to identify urgent PRs
prctrl timeline --priority

# Show timelines for recent PRs only (last 7 days)
prctrl timeline --since-days 7

# Combine filters
prctrl timeline --repo myorg --author reviewer1 --since-days 14
```

## Output

```
════════════════════════════════════════════════════════════
📜 PR #4821 — feat: add CSV export Timeline
════════════════════════════════════════════════════════════

  📊 Summary: 3 reviews, 7 comments, 2 label changes, 3 other events
  ────────────────────────────────────────────────────────────

  2026-03-15 09:30  📣  PR marked as ready for review by @sarah_dev

  2026-03-15 09:31  👥  Review requested from @reviewer1

  2026-03-15 09:31  👥  Review requested from @reviewer2

  2026-03-15 14:20  💬  Comment by @reviewer1: "Looking good overall..."

  2026-03-16 10:15  🔁  CHANGES_REQUESTED by @reviewer1 review: "Please fix the..."

  2026-03-17 11:00  🏷️  Labeled with *needs-changes* by @reviewer1

  2026-03-18 16:30  💬  Comment by @sarah_dev: "Fixed! PTAL"

  2026-03-19 09:00  ✅  APPROVED by @reviewer1 review: "LGTM now"

  2026-03-19 09:05  ✅  APPROVED by @reviewer2 review

  ────────────────────────────────────────────────────────────
  🔗 https://github.com/myorg/frontend/pull/4821
════════════════════════════════════════════════════════════
```

## Event Types

The timeline captures various event types:

| Icon | Event | Description |
|------|-------|-------------|
| `📣` | `ready_for_review` | PR marked as ready for review |
| `👥` | `review_requested` | Review requested from user/team |
| `✅` | `PullRequestReview` (APPROVED) | PR approved |
| `🔁` | `PullRequestReview` (CHANGES_REQUESTED) | Changes requested |
| `💬` | `Comment` / `IssueComment` | General comment |
| `🏷️` | `labeled` / `unlabeled` | Label added/removed |
| `👤` | `assigned` / `unassigned` | Assignee changed |
| `⚡` | `head_ref_force_pushed` | Branch force-pushed |
| `🔀` | `merged` | PR merged |
| `❌` | `closed` | PR closed (not merged) |
| `🔄` | `reopened` | PR reopened |
| `🔒` / `🔓` | `locked` / `unlocked` | PR locked/unlocked |

## JSON Output

When `--json` is specified, outputs a structured JSON array of events:

```json
{
  "pr_number": 4821,
  "pr_title": "feat: add CSV export",
  "repo": "myorg/frontend",
  "url": "https://github.com/myorg/frontend/pull/4821",
  "priority_score": 4,
  "events": [
    {
      "event": "PullRequestReview",
      "created_at": "2026-03-19T09:00:00Z",
      "actor": "reviewer1",
      "data": {
        "review_state": "APPROVED",
        "body_preview": "LGTM now"
      }
    },
    ...
  ]
}
```

## Tips

- Use `--json` for integration with scripts, dashboards, or custom tooling
- Timeline shows events chronologically (oldest first)
- Summary shows counts of reviews, comments, and label changes
- Great for understanding why a PR has been sitting — check the timeline to see if it's actively being worked on or abandoned

## Related Commands

- [`info`](./info.md) — Show full PR details (description, labels, reviewers)
- [`activity`](./activity.md) — Show your recent review activity across all PRs
- [`chase`](./chase.md) — Send follow-up reminders to stale PR authors
