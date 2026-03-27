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
review-dispatcher ping [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-e, --emoji <EMOJI>` | Reaction emoji to send | `eyes` |
| `-s, --send` | Actually send the reaction (preview by default) | `false` |
| `-a, --all` | Ping all pending reviews | `false` |
| `PR_NUMBERS` | PR number(s) to ping (comma-separated) | Interactive |

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

  🔍 Would send  #4821 — Fix authentication bug by @alice (3 days old)
    Preview only — use `--send` to actually ping

──────────────────────────────────────────────────
  💡 Use `--send` to actually send the emoji reactions
  💡 Available emojis: eyes (default), rocket, heart, +1, hooray
  💡 Use `-e rocket` or `-e heart` to change emoji
```

**Send mode:**
```
👀 Ping Command
──────────────────────────────────────────────────
  Emoji: rocket

  📤 Sending  #4821 — Fix authentication bug by @alice (3 days old)
    ⏳ Reacting... ✅ Done!

──────────────────────────────────────────────────
```

## Examples

```bash
# Preview what would happen (default)
review-dispatcher ping

# Ping specific PRs (interactive selection)
review-dispatcher ping 4821
review-dispatcher ping 4821,4815,4809

# Send 👀 (eyes) emoji — default
review-dispatcher ping 4821 --send

# Send 🚀 (rocket) for urgent follow-up
review-dispatcher ping --emoji rocket --send

# Ping all pending reviews at once
review-dispatcher ping --all --send

# Use thumbs up 👍 instead
review-dispatcher ping -e +1 4821 --send
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
