# follow

**Watch PRs for status changes and get notified when something changes.**

When you're tracking PRs you've delegated, reviewed, or are otherwise interested in, `follow` monitors them for changes like CI passing, approval, merge conflicts, or new commits.

## When to Use

- Track delegated PRs you've sent for review
- Watch a PR you're waiting to merge
- Monitor PRs where you need to know when CI passes
- Keep tabs on PRs without cluttering your review queue

## Synopsis

```bash
review-dispatcher follow <ACTION> [OPTIONS]
```

## Actions

| Action | Description |
|--------|-------------|
| `add` | Add PR(s) to your follow list |
| `list` | Show all PRs you're currently following |
| `remove` | Remove PR(s) from your follow list |
| `clear` | Clear all followed PRs |
| `status` | Check for status changes since last check |

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `<PR_NUMBER>` | PR number to follow (shorthand for `--pr`) | Optional |
| `-p, --pr <NUM>` | PR number to follow (shorthand for `--pr-numbers` with single value) | Optional |
| `--pr-number <NUM>` | PR number to follow (shorthand for `--pr-numbers` with single value) | Optional |
| `--pr-numbers <NUMS>` | PR number(s) to follow (comma-separated, format: `repo#123` or `123`) | Optional |
| `--json` | Output as JSON for scripting | `false` |
| `--repo` | Filter by repository name (partial match, case-insensitive). For `add`: filters which PRs are shown in interactive picker. For `list`/`remove`: filters followed PRs. | `none` |
| `--author` | Filter by author username (partial match, case-insensitive). For `add`: filters which PRs are shown in interactive picker. For `list`/`remove`: filters followed PRs. | `none` |
| `--priority` / `-P` | Show priority indicator based on PR size (🟢 SMALL, 🟡 MEDIUM, 🔴 LARGE) | `false` |
| `--since-days` / `-s` | Only show PRs created since this many days ago (for `add`: filters pending reviews; for `list`/`remove`: filters followed PRs by follow date) | `none` |

## PR Format

PRs can be specified in two ways:

- **Full format**: `repo#123` (e.g., `frontend#4821`)
- **Short format**: `123` (uses first repo from config)

## Examples

```bash
# Add a PR to your follow list
review-dispatcher follow add 123

# Follow a PR using --pr flag (consistent with other commands)
review-dispatcher follow add --pr 123

# Follow a PR using --pr-numbers flag
review-dispatcher follow add --pr-numbers 123

# Follow multiple PRs
review-dispatcher follow add 123,456,789

# Follow PRs across different repos
review-dispatcher follow add frontend#4821,backend#1024

# Add: Show only PRs from a specific repo in interactive picker
review-dispatcher follow add --repo frontend

# Add: Show only PRs from a specific author in interactive picker
review-dispatcher follow add --author alice

# Add: Filter by both repo and author
review-dispatcher follow add --repo frontend --author alice

# Add: Follow only recent PRs (created in last 7 days)
review-dispatcher follow add --since-days 7

# List all followed PRs
review-dispatcher follow list

# List followed PRs with priority indicators
review-dispatcher follow list --priority

# List followed PRs as JSON (for scripting)
review-dispatcher follow list --json

# List followed PRs for a specific repo
review-dispatcher follow list --repo frontend

# List followed PRs by a specific author
review-dispatcher follow list --author alice

# List followed PRs filtering by both repo and author
review-dispatcher follow list --repo frontend --author alice

# Remove a PR from following
review-dispatcher follow remove 123

# Remove all followed PRs by a specific author
review-dispatcher follow remove --author alice

# Clear all followed PRs
review-dispatcher follow clear

# Check for status changes
review-dispatcher follow status

# Check for status changes for a specific repo
review-dispatcher follow status --repo backend

# Check for changes and output as JSON
review-dispatcher follow status --json
```

## Tracked Changes

The `status` command detects:

| Change | What It Means |
|--------|---------------|
| **Status changed** | PR opened → merged/closed/draft |
| **New commit** | Author pushed new changes |
| **CI status** | CI pipeline passed/failed/pending |

## Output

```
👁️  Following 3 PR(s)
──────────────────────────────────────────────────
🟢 myorg/frontend #123 — Add user authentication  🟢 SMALL (365 lines)
    📊 +340/-25 lines  |  CI: ⏳  |  Review: ─  |  Author: alice
🟢 myorg/backend #456 — Fix database connection pool  🔴 LARGE (1400 lines)
    📊 +1200/-200 lines  |  CI: ✅  |  Review: 🔁  |  Author: bob
📝 myorg/frontend #789 — WIP: Dark mode support  🟢 SMALL (60 lines)
    📊 +50/-10 lines  |  CI: ✅  |  Review: ✅  |  Author: carol
```

## Status Check Output

```
🔍 Checking status of 3 followed PR(s)...

  🔔 myorg/frontend #123 — Add user authentication
      Status: open → merged

  🔔 myorg/backend #456 — Fix database connection pool
      CI: pending → success

  ✅ No changes detected in followed PRs.
```

## JSON Output

When using `--json`, the output is structured for scripting:

**`follow list --json`** returns an array of followed PRs:
```json
[
  {
    "repo": "myorg/frontend",
    "pr_number": 123,
    "pr_title": "Add user authentication",
    "pr_url": "https://github.com/myorg/frontend/pull/123",
    "followed_at": "2026-03-27T10:00:00Z",
    "last_check": "2026-03-28T02:00:00Z",
    "last_known_state": "open",
    "last_ci_status": "pending",
    "last_review_state": "none",
    "last_commit_sha": "abc1234",
    "additions": 340,
    "deletions": 25,
    "author": "alice",
    "draft": false
  }
]
```

**`follow status --json`** returns an array of changes detected:
```json
[
  {
    "repo": "myorg/frontend",
    "pr_number": 123,
    "pr_title": "Add user authentication",
    "state_changed": true,
    "old_state": "open",
    "new_state": "merged",
    "ci_changed": false,
    "old_ci": "pending",
    "new_ci": "pending",
    "has_new_commit": false,
    "old_commit": "abc1234",
    "new_commit_sha": "abc1234"
  }
]
```

## How It Works

1. **Add** stores PR metadata (title, URL, author, size, last known state) in `.followed.json`
2. **Status** fetches current state and compares with stored state
3. Changes are reported and the stored state is updated
4. No notifications are sent (just local comparison) — combine with `monitor` for notifications

## Storage

Followed PRs are stored in:
```
<review_output_dir>/.followed.json
```

## Related Commands

- [`monitor`](./monitor.md) — Continuous monitoring with macOS notifications
- [`digest`](./digest.md) — Daily digest of pending reviews
- [`list`](./list.md) — List all pending reviews
