# team-summary

**Show team review summary — how many PRs each crew member has waiting.**

See the review load across your team.

## When to Use

- Team standup: "Who has the most reviews?"
- Load balancing: "Can someone help?"

## Synopsis

```bash
prctrl team-summary [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-a, --all` | Show team summary for all pending reviews without interactive selection | `false` |
| `PR_NUMBER` | Filter to specific PR (shorthand for `--pr`) | - |
| `-p, --pr <NUMBER>` | Filter to a specific PR number | - |
| `--pr-numbers <NUMS>` | Show team summary for specific PR numbers (comma-separated) | - |
| `--json` | Output as JSON for scripting | `false` |
| `--repo <NAME>` | Filter by repository (partial match, case-insensitive) | - |
| `--author <NAME>` | Filter by author (partial match, case-insensitive) | - |
| `-P, --priority` | Show priority breakdown (stars by age/size score) | `false` |
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | - |

**Note:** The global `--pr` flag (`-p`) also works with this command for consistency with other commands.

## Examples

```bash
# Human-readable output
prctrl team-summary

# JSON output for scripting
prctrl team-summary --json

# Team summary for a specific repo
prctrl team-summary --repo frontend

# Team summary filtered by author
prctrl team-summary --author alice

# Team summary with priority breakdown
prctrl team-summary --priority

# Team summary for a specific PR
prctrl team-summary 4821
prctrl team-summary --pr 4821

# Team summary for PRs created in the last 7 days only
prctrl team-summary --since-days 7
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
