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
review-dispatcher focus [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-o, --open` | Open the focused PR in your browser | `false` |
| `--json` | Output as JSON (includes full PR details) | `false` |

## Examples

```bash
# See your most urgent PR right now
review-dispatcher focus

# Open it directly in your browser
review-dispatcher focus --open

# Get full details as JSON for scripting
review-dispatcher focus --json
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
- Priority score (1-5 stars)
- Direct link to the PR

When run with `--open`, it opens the PR directly in your default browser.

## Tips

- Pair with `claim` to assign yourself to the focused PR immediately
- Use `--open` to jump straight into reviewing without copy-pasting URLs
- If you have no pending reviews, you'll see an encouraging "You're all clear!" message
