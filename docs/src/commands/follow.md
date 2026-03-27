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
| `<PR_NUMBERS>` | PR number(s) to follow (comma-separated, format: `repo#123` or `123`) | Required for `add`/`remove` |
| `--json` | Output as JSON for scripting | `false` |

## PR Format

PRs can be specified in two ways:

- **Full format**: `repo#123` (e.g., `frontend#4821`)
- **Short format**: `123` (uses first repo from config)

## Examples

```bash
# Add a PR to your follow list
review-dispatcher follow add 123

# Follow multiple PRs
review-dispatcher follow add 123,456,789

# Follow PRs across different repos
review-dispatcher follow add frontend#4821,backend#1024

# List all followed PRs
review-dispatcher follow list

# Remove a PR from following
review-dispatcher follow remove 123

# Clear all followed PRs
review-dispatcher follow clear

# Check for status changes
review-dispatcher follow status
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
🟢 myorg/frontend #123 — Add user authentication
    📊 +340/-25 lines  |  CI: ⏳  |  Review: ─  |  Author: alice
🟢 myorg/backend #456 — Fix database connection pool
    📊 +1200/-200 lines  |  CI: ✅  |  Review: 🔁  |  Author: bob
📝 myorg/frontend #789 — WIP: Dark mode support
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
