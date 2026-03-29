# conflicts

**Show which pending PRs have merge conflicts.**

Avoid wasting time on PRs that can't be merged yet.

## When to Use

- Pre-review: "Is this even mergeable?"
- Sprint planning: "Which PRs are blocked?"
- Merge day: "What can we actually ship?"

## Synopsis

```bash
review-dispatcher conflicts [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--conflicts-only`, `-c` | Hide PRs without conflicts | `false` |
| `--all`, `-a` | Check conflict status for all pending reviews | `false` |
| `--pr-numbers` | Check conflict status for specific PRs (comma-separated) | - |
| `--pr` | Target a specific PR by number | - |
| `--repo` | Filter by repository name (partial match, case-insensitive) | - |
| `--author` | Filter by author username (partial match, case-insensitive) | - |
| `--since-days`, `-s` | Only show PRs created since this many days ago | - |
| `--priority`, `-P` | Show priority scores for each PR (1-5 stars based on age and size) | `false` |
| `--json` | Output as JSON | `false` |

## Examples

```bash
# Check all pending PRs for conflicts
review-dispatcher conflicts

# Show only PRs with conflicts
review-dispatcher conflicts --conflicts-only

# Check conflict status for specific PRs
review-dispatcher conflicts --pr-numbers 123,456,789

# Check a specific PR
review-dispatcher conflicts --pr 123

# Check conflict status for all pending reviews
review-dispatcher conflicts --all

# Check conflicts for a specific repo
review-dispatcher conflicts --repo myservice

# Check conflicts for PRs by a specific author
review-dispatcher conflicts --author johndoe

# Show priority scores alongside conflict status
review-dispatcher conflicts --priority

# Check conflicts for PRs created in the last 3 days
review-dispatcher conflicts --since-days 3
```
