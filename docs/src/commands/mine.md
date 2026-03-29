# mine

**List your own open PRs — draft or not.**

Track your own pull requests from the command line.

## When to Use

- Status check: "What's my open PRs?"
- Draft management: "Find all my draft PRs"

## Synopsis

```bash
prctrl mine [flags]
```

## Flags

- `--json` — Output as JSON (useful for scripting)
- `-a, --all` — Show all matching PRs at once (without prompting)
- `-P, --priority` — Show priority score (1-5 stars) based on age and size
- `-s, --since-days <DAYS>` — Only show PRs created since this many days ago
- `--repo <REPO>` — Filter by repository name (partial match, case-insensitive)
- `--author <AUTHOR>` — Filter by author username (partial match, case-insensitive)
- `-p, --pr <PR_NUMBER>` — Target a specific PR by number
- `--pr-numbers <NUMBERS>` — Target multiple PRs by number (comma-separated, e.g., `123,456,789`)

## Global Flags

These flags are available globally and work with `mine`:

- `-p, --pr <PR_NUMBER>` — Target a specific PR by number (overrides filters)
- `--include-drafts` — Include draft PRs in results
- `--exclude-prefix <PREFIXES>` — Exclude PRs with matching title prefixes (comma-separated)
- `-o, --output-dir <PATH>` — Folder for review files (default: ./reviews)

## Snooze Behavior

Snoozed PRs are automatically hidden from `mine` results (consistent with `list` and `delegate`). Use `--pr`, `--pr-number`, or `--pr-numbers` to bypass this filter and view specific snoozed PRs.

## Examples

```bash
# List your open PRs
prctrl mine

# Show all PRs at once (non-interactive)
prctrl mine --all

# Show your PRs with priority scores
prctrl mine --priority

# Get JSON output for scripting
prctrl mine --json

# Only show PRs from the last 7 days
prctrl mine --since-days 7

# Filter by repository
prctrl mine --repo my-repo

# Filter by author
prctrl mine --author johndoe

# Target a specific PR (bypasses snooze filter)
prctrl mine --pr 123

# Target multiple specific PRs
prctrl mine --pr-numbers 123,456,789

# Combine filters
prctrl mine --since-days 14 --repo api --priority

# Combine with global flags
prctrl mine --include-drafts --priority
```
