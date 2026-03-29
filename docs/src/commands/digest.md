# digest

**Generate a shareable weekly digest of your pending PR reviews — perfect for Slack, Teams, or email.**

Unlike `report` (which shows what you've already reviewed), `digest` gives you a snapshot of what's currently waiting, grouped by age, repository, and author.

## When to Use

- Monday morning: "Let me share the team digest in Slack"
- Before a standup: "Quick status check for the channel"
- Weekly wrap-up: "Here's what's pending heading into the weekend"
- Stakeholder update: "Here's a plain-text summary of review load"

## Synopsis

```bash
review-dispatcher digest [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-d, --days <DAYS>` | Number of days to include | `7` |
| `--raw` | Output as raw Markdown (for Slack/Teams) | `false` |
| `--repo <REPO>` | Filter by repository name (partial match, case-insensitive) | |
| `--author <AUTHOR>` | Filter by author username (partial match, case-insensitive) | |
| `-P, --priority` | Show priority scores and most urgent PR | `false` |
| `-s, --since-days <DAYS>` | Only show PRs created in the last N days (newer PRs) | |
| `--older-than <DAYS>` | Only show PRs older than N days (neglected PRs) | |

## Examples

```bash
# Pretty terminal digest
review-dispatcher digest

# Shorter lookback (last 3 days)
review-dispatcher digest --days 3

# Raw Markdown for pasting into Slack
review-dispatcher digest --raw

# Raw Markdown with custom window
review-dispatcher digest --days 14 --raw

# Filter by repository
review-dispatcher digest --repo api

# Filter by author
review-dispatcher digest --author alice

# Show priority scores and most urgent PR
review-dispatcher digest --priority

# Combined: raw markdown filtered by repo with priority
review-dispatcher digest --raw --repo backend --priority

# Only show PRs created in the last 3 days (recent PRs)
review-dispatcher digest --since-days 3

# Only show PRs older than 7 days (neglected PRs)
review-dispatcher digest --older-than 7
```

## Output Examples

### Terminal output

```
📋 Weekly Review Digest — last 7 days
─────────────────────────────────────────────

  📊 Summary
     Total PRs:          12
     Lines changed:      +2847 / -1203
     🚨 Overdue (15d+):  2

  ⏱️  Age Breakdown
     🆕 New: 3
     🌱 Fresh: 4
     ⏳ Aging: 2
     🔥 Stale: 1

  📁 By Repository (top 5)
     api-gateway: 4
     frontend: 3
     shared-libs: 2

  👥 By Author (top 5)
     alice: 4
     bob: 3
     carol: 2

  🚨 Overdue PRs (15d+)
     #4821 fix: critical auth bug — 18d old
     #4798 refactor: legacy cache layer — 16d old

─────────────────────────────────────────────
  💡 Use `--raw` to get Markdown output for Slack/Teams
```

### With `--priority` flag

```
📋 Weekly Review Digest — last 7 days
─────────────────────────────────────────────

  📊 Summary
     Total PRs:          12
     Lines changed:      +2847 / -1203
     🚨 Overdue (15d+):  2

  🚨 Most Urgent:
    fix: critical auth bug  #4821  ⭐⭐⭐⭐⭐
    👤 alice  •  📦 1247 lines  •  ⏱️ 18d  •  api-gateway

  ⭐ Priority Breakdown:
     ⭐⭐⭐⭐⭐  2 PR(s)  •  oldest: 18d  •  +2340/-892 lines
     ⭐⭐⭐⭐   3 PR(s)  •  oldest: 10d  •  +847/-423 lines
     ⭐⭐⭐    4 PR(s)  •  oldest: 5d  •  +432/-156 lines
     ⭐⭐     2 PR(s)  •  oldest: 2d  •  +156/-89 lines
     ⭐      1 PR(s)  •  oldest: 0d  •  +72/-43 lines

─────────────────────────────────────────────
  💡 Use `--raw` to get Markdown output for Slack/Teams
  💡 Use `--priority` to show priority scores and most urgent PR
```

### Raw Markdown output (`--raw`)

```markdown
## 📋 Review Digest — last 7 days

**Total:** 12 PRs | **+2847** / **-1203** lines | **Overdue:** 2

### By Repository
- **api-gateway**: 4 PR(s)
- **frontend**: 3 PR(s)
- **shared-libs**: 2 PR(s)

### By Author
- **alice**: 4 PR(s)
- **bob**: 3 PR(s)
- **carol**: 2 PR(s)

### Age Breakdown
- 🆕 **New**: 3 PR(s)
- 🌱 **Fresh**: 4 PR(s)
- ⏳ **Aging**: 2 PR(s)
- 🔥 **Stale**: 1 PR(s)

### 🚨 Overdue (15d+)
- [api-gateway#4821](https://github.com/org/api-gateway/pull/4821) *fix: critical auth bug* — 18d old
- [shared-libs#4798](https://github.com/org/shared-libs/pull/4798) *refactor: legacy cache layer* — 16d old
```

## Tips

- Use `--raw` when posting to Slack, Teams, or email — the Markdown renders nicely in all three
- Pipe to `pbcopy` to copy to clipboard: `review-dispatcher digest --raw | pbcopy`
- Use `--days 1` for a daily standup digest instead of weekly
- Use `--priority` to identify the most urgent PR at a glance
- Age buckets: 🆕 New 0-1d · 🌱 Fresh 2-3d · ⏳ Aging 4-7d · 🔥 Stale 8-14d · 💀 Overdue 15d+
- Combine filters for targeted digests: `digest --repo backend --author alice --priority`
- Use `--since-days 7` to show only recently created PRs
- Use `--older-than 7` to focus on neglected PRs waiting 7+ days
- `--since-days` and `--older-than` can be combined but `--since-days` takes precedence
