# files

**Show changed files for one or more PRs.**

Quickly see which files were modified before diving into the diff.

## When to Use

- Pre-review scan: "What did they change?"
- Impact assessment: "Which services are affected?"

## Synopsis

```bash
prctrl files [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to show files for (shorthand for `--pr`) | - |
| `-p, --pr <PR>` | Show files for specific PR (bypasses snooze exclusion) | - |
| `--pr-numbers <NUMS>` | PR number(s) to show files for (comma-separated) | - |
| `-a, --all` | Show files for all pending reviews | `false` |
| `-P, --priority` | Show priority scores for each PR (1-5 stars) | `false` |
| `--json` | Output as JSON (useful for scripting) | `false` |
| `--repo <REPO>` | Filter by repository name (partial match, case-insensitive) | - |
| `--author <USER>` | Filter by author username (partial match, case-insensitive) | - |
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | - |

## Examples

```bash
# Show files for a specific PR
prctrl files 4821
prctrl files --pr 4821

# Show files for multiple PRs
prctrl files --pr-numbers 4821,3156,2890

# Show files for all pending reviews
prctrl files --all

# Filter by repository
prctrl files --all --repo myservice

# Filter by author
prctrl files --all --author johndoe

# Combined filters
prctrl files --all --repo api --priority

# Only show files for recent PRs (last 7 days)
prctrl files --all --since-days 7

# JSON output for scripting
prctrl files --pr 4821 --json
```

## JSON Output

When `--json` is specified, output includes for each PR:

- `pr_number`, `pr_title`, `repo`, `url`
- `total_files`, `total_additions`, `total_deletions`
- `files[]` - array of file objects with `filename`, `status`, `additions`, `deletions`

## Notes

- Snoozed PRs are automatically excluded when using `--all` or interactive selection (consistent with `search`, `top`, `delegate`, and `list` commands)
- Use `--pr` to bypass snooze exclusion and view files for a specific snoozed PR
