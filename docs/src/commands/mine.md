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

## Examples

```bash
# List your open PRs
review-dispatcher mine

# Show your PRs with priority scores
review-dispatcher mine --priority

# Get JSON output for scripting
review-dispatcher mine --json

# Combine with global flags
review-dispatcher mine --include-drafts --priority
```
