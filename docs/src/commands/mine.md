# mine

**List your own open PRs — draft or not.**

Track your own pull requests from the command line.

## When to Use

- Status check: "What's my open PRs?"
- Draft management: "Find all my draft PRs"

## Synopsis

```bash
review-dispatcher mine [flags]
```

## Flags

- `--json` — Output as JSON (useful for scripting)
- `-a, --all` — Show all matching PRs at once (without prompting)
- `-P, --priority` — Show priority score (1-5 stars) based on age and size
- `-s, --since-days <DAYS>` — Only show PRs created since this many days ago
- `--repo <REPO>` — Filter by repository name (partial match, case-insensitive)
- `--author <AUTHOR>` — Filter by author username (partial match, case-insensitive)
- `-p, --pr <PR_NUMBER>` — Target a specific PR by number

## Global Flags

These flags are available globally and work with `mine`:

- `-p, --pr <PR_NUMBER>` — Target a specific PR by number (overrides filters)
- `--include-drafts` — Include draft PRs in results
- `--exclude-prefix <PREFIXES>` — Exclude PRs with matching title prefixes (comma-separated)
- `-o, --output-dir <PATH>` — Folder for review files (default: ./reviews)

## Snooze Behavior

Snoozed PRs are automatically hidden from `mine` results (consistent with `list` and `delegate`). Use `--pr` to bypass this filter and view a specific snoozed PR.

## Examples

```bash
# List your open PRs
review-dispatcher mine

# Show all PRs at once (non-interactive)
review-dispatcher mine --all

# Show your PRs with priority scores
review-dispatcher mine --priority

# Get JSON output for scripting
review-dispatcher mine --json

# Only show PRs from the last 7 days
review-dispatcher mine --since-days 7

# Filter by repository
review-dispatcher mine --repo my-repo

# Filter by author
review-dispatcher mine --author johndoe

# Target a specific PR (bypasses snooze filter)
review-dispatcher mine --pr 123

# Combine filters
review-dispatcher mine --since-days 14 --repo api --priority

# Combine with global flags
review-dispatcher mine --include-drafts --priority
```
