# Browse

**Open one or more PRs in your browser.**

Jump straight to GitHub without switching windows or copying URLs.

> **Note:** Snoozed PRs are automatically excluded from results (unless you specify a specific PR with `--pr` or `--pr-numbers`). This is consistent with other commands like `list` and `delegate`.

## When to Use

- After triage: "Let me see the actual code"
- Quick access: "Open all my pending PRs"

## Synopsis

```bash
prctrl browse [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to open (shorthand for `--pr`) | - |
| `-p, --pr <NUM>` | Open specific PR (shorthand for `--pr`) | - |
| `--pr-numbers <NUMS>` | PR number(s) to open (comma-separated) | - |
| `-a, --all` | Open all pending reviews | `false` |
| `-n, --dry-run` | Preview which PRs would be opened without opening them | `false` |
| `-q, --quiet` | Suppress per-PR success/failure messages (show only summary) | `false` |
| `-P, --priority` | Show priority scores for each PR (1-5 stars based on age and size) | `false` |
| `--json` | Output URLs as JSON (without opening browser) | `false` |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | - |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | - |
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | - |

**Note:** The global `--pr` flag (`-p`) also works with this command for consistency with other commands.

## Examples

```bash
# Open a specific PR in browser
prctrl browse 4821
prctrl browse --pr 4821

# Open multiple PRs in browser
prctrl browse --pr-numbers 4821,3156,2890

# Open multiple PRs from a specific repository (filters applied after fetch)
prctrl browse --pr-numbers 4821,3156,2890 --repo myservice

# Open all pending reviews at once
prctrl browse --all

# Open all pending reviews from a specific repository
prctrl browse --all --repo myservice

# Open all pending reviews from a specific author
prctrl browse --all --author johndoe

# Open all pending reviews from a repository by an author
prctrl browse --all --repo myservice --author johndoe

# Open recent PRs (last 7 days) from a repository
prctrl browse --all --repo myservice --since-days 7

# Open old PRs needing attention (more than 7 days old)
prctrl browse --all --since-days 30

# Preview which PRs would be opened (dry-run)
prctrl browse --dry-run

# Preview opening specific PRs
prctrl browse --pr-numbers 4821,3156 --dry-run

# Output URLs as JSON (useful for scripting)
prctrl browse --pr 4821 --json

# Quiet mode - open PRs with minimal output
prctrl browse --all --quiet

# Show priority scores when browsing (helps decide which PRs to open first)
prctrl browse --all --priority

# Show priority scores in JSON output for scripting
prctrl browse --all --json --priority
```
