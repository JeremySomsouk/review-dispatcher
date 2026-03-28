# blocked

**Show PRs that are blocked from merging — CI failures, merge conflicts, or other issues.**

Find PRs that can't be merged yet and see exactly what's blocking them.

## When to Use

- Merge queue planning: "What's blocking our PRs?"
- CI debugging: "Why is this PR blocked?"
- Conflicts check: "Do we have merge conflicts to resolve?"

## Synopsis

```bash
review-dispatcher blocked [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-r, --repo <NAME>` | Filter to specific repository (partial match, case-insensitive) | All repos |
| `--author <NAME>` | Filter by author username (partial match, case-insensitive) | All authors |
| `-c, --ci-only` | Only show PRs with failing CI | `false` |
| `-m, --conflicts-only` | Only show PRs with merge conflicts | `false` |
| `-P, --priority` | Show priority scores (1-5 stars based on age and size) | `false` |
| `-n, --limit <NUM>` | Maximum results shown | `20` |
| `--json` | Output as JSON | `false` |

## Examples

```bash
# Show all blocked PRs
review-dispatcher blocked

# Show only PRs with CI failures
review-dispatcher blocked --ci-only

# Show only PRs with merge conflicts
review-dispatcher blocked --conflicts-only

# Filter to specific repo
review-dispatcher blocked --repo frontend

# Filter by author
review-dispatcher blocked --author alice

# Show priority scores
review-dispatcher blocked --priority

# Combine filters
review-dispatcher blocked --repo frontend --priority --ci-only

# JSON output for scripting
review-dispatcher blocked --json
```

## Output Details

Each blocked PR shows:
- **PR number and title**: Which PR is blocked
- **Blocker reasons**: Why it can't be merged (CI failing, conflicts, draft, etc.)
- **Author and age**: Who owns it and how long it's been waiting
- **Priority score**: When `--priority` is used, shows urgency rating
