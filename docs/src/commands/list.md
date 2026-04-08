# list

**List all PRs waiting for your review or where you've commented.**

This is your starting point. Shows every PR where you're requested as a reviewer, plus PRs where you've already commented (so you can easily revisit your feedback). Sorted by oldest first.

## When to Use

- Morning check: "What needs my attention today?"
- Before a meeting: "Any urgent PRs I should know about?"
- After returning from vacation: "What did I miss?"

## Synopsis

```bash
prctrl list [OPTIONS]
prctrl ls [OPTIONS]     # shorthand alias
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--json` | Output as JSON (great for scripting) | `false` |
| `-s, --since-days <DAYS>` | Only show PRs from the last N days | all |
| `-P, --priority` | Add priority scores (1-5 stars based on age + size) | `false` |
| `--repo <NAME>` | Filter by repository (partial match, case-insensitive) | - |
| `--author <NAME>` | Filter by author (partial match, case-insensitive) | - |
| `-p, --pr <NUMBER>` | Show specific PR by number | - |
| `--pr-numbers <NUMBERS>` | Show specific PR(s) by number (comma-separated) | - |
| `--commented` | Show only PRs where you have already commented | `false` |
| `--show-stacks` | Show stacked PRs (PRs that build on each other) | `false` |

## Examples

```bash
# See everything waiting for you
prctrl list

# Find recent PRs only (last 7 days)
prctrl list --since-days 7

# With priority scores (oldest + largest = highest priority)
prctrl list --priority

# Filter to one repo
prctrl list --repo frontend

# Filter by author
prctrl list --author alice

# Combine filters
prctrl list --repo api --author bob --priority

# JSON output for scripts
prctrl list --json | jq '.[] | select(.author == "alice")'

# Show specific PR by number (via global --pr flag or local flag)
prctrl list --pr 4821
prctrl list -p 4821

# Show multiple PRs by number
prctrl list --pr-numbers 4821,3156

# Show only PRs where you have commented
prctrl list --commented

# Show PRs with stacked PR detection
prctrl list --show-stacks

# Combine with other filters
prctrl list --repo api --show-stacks --priority
```

## Output Example

```
🔍 4 pending review(s)

[1] feat: add dark mode          #4821 (frontend)  👤 alice  +89   2 days
[2] fix: login timeout           #3156 (backend)   👤 bob    +234  4 days ⭐⭐⭐
[3] refactor: clean API          #2890 (shared)    👤 carol  +12   1 day
[4] chore: bump deps             #4521 (deps)      👤 dave   +890  5 days
```

## Tips

- PRs are sorted oldest-first so you don't miss stale reviews
- Use `--priority` to surface the most urgent ones visually
- Combine with `--repo` to focus on one codebase at a time
- Use `--show-stacks` to see PRs that build on each other (stacked PRs)

## Stack Detection

When `--show-stacks` is enabled, `list` detects **stacked PRs** — PRs where one PR's base branch is another PR's head branch. This helps identify dependent PRs that need to be reviewed in sequence.

Example stack output:
```
┌─ Stack on `main` (3 PRs)

🔵 #123 - Add new feature
  └─ @feature
    https://github.com/owner/repo/pull/123

  #124 - Implement API endpoint
  └─ @feature-2
    https://github.com/owner/repo/pull/124

  #125 - Add tests
  └─ @feature-3
    https://github.com/owner/repo/pull/125
```

Stack detection works by analyzing branch relationships:
- PR targeting branch `feature` is the base
- PR targeting branch `feature-2` builds on it
- This creates a stack that should be reviewed in order
