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
- `-P, --priority` — Show priority score (1-5 stars) based on age and size
- `-s, --since-days <DAYS>` — Only show PRs created since this many days ago
- `--repo <REPO>` — Filter by repository name (partial match, case-insensitive)
- `--author <AUTHOR>` — Filter by author username (partial match, case-insensitive)

## Examples

```bash
# List your open PRs
review-dispatcher mine

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

# Combine filters
review-dispatcher mine --since-days 14 --repo api --priority

# Combine with global flags
review-dispatcher mine --include-drafts --priority
```
