# browse

**Open one or more PRs in your browser.**

Jump straight to GitHub without switching windows or copying URLs.

## When to Use

- After triage: "Let me see the actual code"
- Quick access: "Open all my pending PRs"

## Synopsis

```bash
review-dispatcher browse [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to open (shorthand for `--pr`) | - |
| `-p, --pr <NUM>` | Open specific PR (shorthand for `--pr`) | - |
| `--pr-numbers <NUMS>` | PR number(s) to open (comma-separated) | - |
| `-a, --all` | Open all pending reviews | `false` |
| `-n, --dry-run` | Preview which PRs would be opened without opening them | `false` |
| `--json` | Output URLs as JSON (without opening browser) | `false` |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | - |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | - |
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | - |

**Note:** The global `--pr` flag (`-p`) also works with this command for consistency with other commands.

## Examples

```bash
# Open a specific PR in browser
review-dispatcher browse 4821
review-dispatcher browse --pr 4821

# Open multiple PRs in browser
review-dispatcher browse --pr-numbers 4821,3156,2890

# Open all pending reviews at once
review-dispatcher browse --all

# Open all pending reviews from a specific repository
review-dispatcher browse --all --repo myservice

# Open all pending reviews from a specific author
review-dispatcher browse --all --author johndoe

# Open all pending reviews from a repository by an author
review-dispatcher browse --all --repo myservice --author johndoe

# Open recent PRs (last 7 days) from a repository
review-dispatcher browse --all --repo myservice --since-days 7

# Open old PRs needing attention (more than 7 days old)
review-dispatcher browse --all --since-days 30

# Preview which PRs would be opened (dry-run)
review-dispatcher browse --dry-run

# Preview opening specific PRs
review-dispatcher browse --pr-numbers 4821,3156 --dry-run

# Output URLs as JSON (useful for scripting)
review-dispatcher browse --pr 4821 --json
```
