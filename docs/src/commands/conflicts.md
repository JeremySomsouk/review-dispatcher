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
| `--repo` | Filter by repository name (partial match, case-insensitive) | - |
| `--author` | Filter by author username (partial match, case-insensitive) | - |
| `--pr` | Target a specific PR by number | - |
| `--json` | Output as JSON | `false` |

## Examples

```bash
# Check all pending PRs for conflicts
review-dispatcher conflicts

# Show only PRs with conflicts
review-dispatcher conflicts --conflicts-only

# Check conflicts for a specific repo
review-dispatcher conflicts --repo myservice

# Check conflicts for PRs by a specific author
review-dispatcher conflicts --author johndoe

# Check a specific PR
review-dispatcher conflicts --pr 123
