# focus

**Show the ONE PR you should focus on right now — the most urgent by priority score.**

When you're overwhelmed by pending reviews, `focus` cuts through the noise and tells you exactly which PR deserves your attention first. It calculates a priority score combining age, size, and urgency, then shows you the single most critical PR.

## When to Use

- "I have 15 minutes — what should I review?"
- Starting your day and want to know where to begin
- After returning from a meeting and need a quick mental reset
- When the list feels overwhelming and you need one clear target

## Synopsis

```bash
prctrl focus [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-n, --dry-run` | Preview which PR would be selected without opening it | `false` |
| `-a, --all` | Show all matching PRs sorted by priority (default: show only the top 1) | `false` |
| `-l, --limit` | Limit the number of PRs shown (use with `--all`, default: 10) | `10` |
| `-o, --open` | Open the focused PR in your browser (only for single PR) | `false` |
| `PR_NUMBER` | Target specific PR by number (shorthand for `--pr`) | - |
| `--pr-numbers` | PR number(s) to focus on (comma-separated) | - |
| `-p, --pr <NUM>` | Target specific PR by number | - |
| `--json` | Output as JSON (one object per line with `--all`) | `false` |
| `-P, --priority` | Show priority score for each PR (1-5 stars) | `false` |
| `--repo` | Filter by repository name (partial match, case-insensitive) | - |
| `--author` | Filter by author username (partial match, case-insensitive) | - |
| `-s, --since-days` | Only show PRs created since this many days ago | - |

## Examples

```bash
# See your most urgent PR right now
prctrl focus

# Preview which PR would be selected (without opening)
prctrl focus --dry-run

# See your top 3 most urgent PRs
prctrl focus --all --limit 3

# Open it directly in your browser
prctrl focus --open

# Get full details as JSON for scripting
prctrl focus --json

# See all high-priority PRs from a specific repository
prctrl focus --all --repo myservice

# Focus on PRs from a specific author
prctrl focus --author johndoe

# Show priority scores with stars
prctrl focus --priority

# Focus only on recently created PRs (last 7 days)
review-dispatcher focus --since-days 7

# Target a specific PR (bypasses priority sorting)
review-dispatcher focus --pr 4821

# Focus on multiple specific PRs
review-dispatcher focus --pr-numbers 4821,4822,4823 --all
```

## Priority Calculation

The focused PR is selected based on a score calculated from:

| Factor | Weight | Description |
|--------|--------|-------------|
| Days waiting | High | Older PRs get higher priority |
| Size | Medium | Larger PRs get higher priority |
| Draft status | Low | Non-draft PRs rank higher |

## Output

When run normally, `focus` shows:
- PR title and number
- Repository
- Author
- Age (days since created)
- Size (total lines changed)
- Priority score (1-5 stars, if `--priority` is set)
- Direct link to the PR

When run with `--open`, it opens the PR directly in your default browser.

## Snooze Behavior

Snoozed PRs are automatically excluded from focus results (consistent with `list`, `delegate`, `top`, `search`, etc.). Use `prctrl snooze add` to temporarily hide PRs from consideration.

## Tips

- Pair with `claim` to assign yourself to the focused PR immediately
- Use `--open` to jump straight into reviewing without copy-pasting URLs
- Use `--repo` and `--author` filters to narrow down which PRs to consider
- If you have no pending reviews, you'll see an encouraging "You're all clear!" message
- Use [`snooze`](./snooze.md) to temporarily hide PRs from results when you need a break from certain reviews
