# ping

**Send emoji reactions to PR authors to get their attention without leaving a comment.**

A lightweight, non-intrusive way to nudge authors — the equivalent of tapping someone's shoulder in a hallway. GitHub shows reactions inline on the PR, so authors notice them immediately without your reaction cluttering the comment thread.

## When to Use

- PR has been waiting for review and you want a gentle reminder
- You want to acknowledge a PR without doing a full review yet
- Following up on a previously reviewed PR
- When `chase` feels too formal but you want to do something

## Synopsis

```bash
prctrl ping [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-e, --emoji <EMOJI>` | Reaction emoji to send | `eyes` |
| `-p, --pr <PR>` | Target specific PR by number (shorthand, use with --send) | - |
| `--pr-number <PR_NUMBER>` | Target specific PR by number | - |
| `--pr-numbers <PR_NUMBERS>` | PR number(s) to ping (comma-separated) | Interactive |
| `-n, --dry-run` | Preview what would be pinged without sending | `false` |
| `-a, --all` | Ping all pending reviews | `false` |
| `--send` | Actually send the reaction (preview by default) | `false` |
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | - |
| `--repo <REPO>` | Filter by repository name (partial match, case-insensitive) | - |
| `--author <AUTHOR>` | Filter by author username (partial match, case-insensitive) | - |
| `--json` | Output as JSON (useful for scripting) | `false` |
| `-P, --priority` | Show priority scores for each PR (1-5 stars based on age and size) | `false` |

## Available Emojis

| Emoji | Name | Use Case |
|-------|------|----------|
| 👀 | `eyes` (default) | "I've seen this, looking into it soon" |
| 🚀 | `rocket` | "This is ready for launch!" or urgent follow-up |
| ❤️ | `heart` | "Love this PR!" or appreciation |
| 👍 | `+1` | Simple approval without full review |
| 🎉 | `hooray` | Celebration / milestone reached |

## How It Works

The `ping` command uses GitHub's emoji reactions API:

1. **Preview mode** (default) — shows what would be sent without actually reacting
2. **Send mode** (`--send`) — actually posts the emoji reaction to the PR

Unlike comments, reactions:
- Don't generate notification spam for the author
- Are visible immediately in the PR's reaction summary
- Can be added by anyone, not just reviewers

## Output

**Preview mode:**
```
👀 Ping Command
──────────────────────────────────────────────────
  Emoji: eyes

  🔍 Will ping #4821 — Fix authentication bug by @alice (3 days old)

──────────────────────────────────────────────────
  💡 Use `--send` to actually send the emoji reactions
  💡 Available emojis: eyes (default), rocket, heart, +1, hooray
  💡 Use `-e rocket` or `-e heart` to change emoji
```

**Dry-run mode:**
```
👀 Ping Command
────────────────────────────────────────────────--
  Emoji: eyes

  🔍 Dry-run: would ping #4821 — Fix authentication bug by @alice (3 days old)

────────────────────────────────────────────────--
  (dry-run — no emoji reactions sent)
```

**JSON output:**
```json
[
  {
    "repo": "my-service",
    "pr_number": 4821,
    "pr_title": "Fix authentication bug",
    "pr_author": "alice",
    "pr_url": "https://github.com/org/my-service/pull/4821",
    "age_days": 3,
    "emoji": "eyes"
  }
]
```

**Send mode:**
```
👀 Ping Command
──────────────────────────────────────────────────
  Emoji: eyes

  📤 Sending #4821 — Fix authentication bug by @alice (3 days old)
  📤 Sending #4822 — Update dependencies by @bob (1 days old)

⏳ Sending 2 emoji reaction(s) in parallel...

  ✅ #4821 — Fix authentication bug
  ✅ #4822 — Update dependencies

📊 Sent: 2, Failed: 0
```

## Examples

```bash
# Preview what would happen (default)
prctrl ping

# Preview with dry-run flag (explicit preview mode)
prctrl ping --dry-run
prctrl ping -n

# Output as JSON (useful for scripting)
prctrl ping --all --json

# Ping specific PRs (interactive selection)
prctrl ping 4821
prctrl ping 4821,4815,4809

# Send 👀 (eyes) emoji — default
prctrl ping 4821 --send

# Ping a specific PR directly with --pr flag
prctrl ping --pr 4821 --send

# Send 🚀 (rocket) for urgent follow-up
prctrl ping --emoji rocket --send

# Ping all pending reviews at once
prctrl ping --all --send

# Use thumbs up 👍 instead
prctrl ping -e +1 4821 --send

# Ping PRs from a specific repository
prctrl ping --repo my-service --all --send

# Ping PRs from a specific author
prctrl ping --author alice --all --send

# Ping PRs created in the last 3 days (newer PRs only)
prctrl ping --since-days 3 --all --send

# Combine filters for targeted pinging
prctrl ping --repo api --author bob --send
```

## Tips

- **Be mindful**: Don't spam the same PR with multiple reactions
- **Pair with `chase`**: Use `ping` for gentle nudges, `chase` for formal reminders
- **Preview first**: Always preview before sending (`--send` is explicit)
- **Rocket emoji**: Great for signaling a PR is ready for final review

## Related Commands

- [`chase`](./chase.md) — Send formal comment reminders
- [`attention`](./attention.md) — Find PRs that need your attention
- [`focus`](./focus.md) — Focus on the most urgent PR
