# list

**List all PRs waiting for your review.**

This is your starting point. Shows every PR where you're requested as a reviewer, sorted by oldest first (so you never miss an aging PR).

## When to Use

- Morning check: "What needs my attention today?"
- Before a meeting: "Any urgent PRs I should know about?"
- After returning from vacation: "What did I miss?"

## Synopsis

```bash
review-dispatcher list [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--json` | Output as JSON (great for scripting) | `false` |
| `-s, --since-days <DAYS>` | Only show PRs from the last N days | all |
| `-P, --priority` | Add priority scores (1-5 stars based on age + size) | `false` |
| `--repo <NAME>` | Filter by repository (partial match) | - |
| `--author <NAME>` | Filter by author (partial match) | - |

## Examples

```bash
# See everything waiting for you
review-dispatcher list

# Find recent PRs only (last 7 days)
review-dispatcher list --since-days 7

# With priority scores (oldest + largest = highest priority)
review-dispatcher list --priority

# Filter to one repo
review-dispatcher list --repo frontend

# JSON output for scripts
review-dispatcher list --json | jq '.[] | select(.author == "alice")'
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
