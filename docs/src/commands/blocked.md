# blocked

**Show PRs that are blocked from merging — CI failures, merge conflicts, or other issues.**

Find PRs that can't be merged yet and see exactly what's blocking them.

## When to Use

- Merge queue planning: "What's blocking our PRs?"
- CI debugging: "Why is this PR blocked?"
- Conflicts check: "Do we have merge conflicts to resolve?"

## Synopsis

```bash
prctrl blocked [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-r, --repo <NAME>` | Filter to specific repository (partial match, case-insensitive) | All repos |
| `--author <NAME>` | Filter by author username (partial match, case-insensitive) | All authors |
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | All PRs |
| `-c, --ci-only` | Only show PRs with failing CI | `false` |
| `-m, --conflicts-only` | Only show PRs with merge conflicts | `false` |
| `-P, --priority` | Show priority scores (1-5 stars based on age and size) | `false` |
| `-n, --limit <NUM>` | Maximum results shown | `20` |
| `--json` | Output as JSON | `false` |

## Examples

```bash
# Show all blocked PRs
prctrl blocked

# Show only PRs with CI failures
prctrl blocked --ci-only

# Show only PRs with merge conflicts
prctrl blocked --conflicts-only

# Filter to specific repo
prctrl blocked --repo frontend

# Filter by author
prctrl blocked --author alice

# Show priority scores
prctrl blocked --priority

# Combine filters
prctrl blocked --repo frontend --priority --ci-only

# Only show recently created PRs (last 7 days)
prctrl blocked --since-days 7

# JSON output for scripting
prctrl blocked --json
```

## Output Details

Each blocked PR shows:
- **PR number and title**: Which PR is blocked
- **Blocker reasons**: Why it can't be merged (CI failing, conflicts, draft, etc.)
- **Author and age**: Who owns it and how long it's been waiting
- **Priority score**: When `--priority` is used, shows urgency rating
