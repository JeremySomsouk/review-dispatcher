# team-summary

**Show team review summary — how many PRs each crew member has waiting.**

See the review load across your team.

## When to Use

- Team standup: "Who has the most reviews?"
- Load balancing: "Can someone help?"

## Synopsis

```bash
review-dispatcher team-summary [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-a, --all` | Show team summary for all pending reviews without interactive selection | `false` |
| `--pr-numbers <NUMS>` | Show team summary for specific PR numbers (comma-separated) | - |
| `PR_NUMBER` | Filter to specific PR (shorthand for `--pr`) | - |
| `--json` | Output as JSON for scripting | `false` |
| `-p, --pr <NUMBER>` | Filter to a specific PR number | - |
| `--repo <NAME>` | Filter by repository (partial match, case-insensitive) | - |
| `--author <NAME>` | Filter by author (partial match, case-insensitive) | - |
| `-P, --priority` | Show priority breakdown (stars by age/size score) | `false` |
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | - |

**Note:** The global `--pr` flag (`-p`) also works with this command for consistency with other commands.

## Examples

```bash
# Human-readable output
review-dispatcher team-summary

# JSON output for scripting
review-dispatcher team-summary --json

# Team summary for a specific repo
review-dispatcher team-summary --repo frontend

# Team summary filtered by author
review-dispatcher team-summary --author alice

# Team summary with priority breakdown
review-dispatcher team-summary --priority

# Team summary for a specific PR
review-dispatcher team-summary 4821
review-dispatcher team-summary --pr 4821

# Team summary for PRs created in the last 7 days only
review-dispatcher team-summary --since-days 7
```

## Sample JSON Output

```json
{
  "total_pending": 12,
  "by_author": {
    "alice": 5,
    "bob": 4,
    "charlie": 3
  },
  "unassigned": 0,
  "by_repository": {
    "myorg/api": 8,
    "myorg/web": 4
  }
}
```
