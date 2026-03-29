# snooze

**Temporarily hide PRs from the pending list.**

Snooze PRs you're not ready to review yet.

## When to Use

- Context overload: "Not now, remind me later"
- Vacation: "Hide until I return"

## Synopsis

```bash
prctrl snooze [OPTIONS] [PR_NUMBER] [PR_NUMBERS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to snooze (shorthand for --pr) | `none` |
| `-p, --pr <NUM>` | Snooze specific PR | `none` |
| `PR_NUMBERS` | PR number(s) to snooze (comma-separated) | `none` |
| `-d, --days <NUM>` | Days to snooze | `3` |
| `-s, --since-days <NUM>` | Only show PRs created since this many days ago | `none` |
| `--json` | Output as JSON (useful for scripting) | `false` |
| `-P, --priority` | Show priority scores for listed snoozed PRs | `false` |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | `none` |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | `none` |

## Subcommands

### list

List all currently snoozed PRs.

```bash
prctrl snooze list
prctrl snooze list --repo myrepo
prctrl snooze list --author johndoe
prctrl snooze list --repo myrepo --author johndoe
```

### review

Show detailed view of snoozed PRs with full metadata.

```bash
prctrl snooze review
prctrl snooze review --repo myrepo
```

### add

Add PR(s) to the snooze list. When using interactive selection (no PR specified), `--repo` and `--author` filters are applied to narrow down the list of pending reviews shown.

```bash
prctrl snooze add 4821
prctrl snooze 4821 --days 7
prctrl snooze add 4821,4822,4823
prctrl snooze add --repo myservice --author johndoe
```

### remove

Remove PR(s) from the snooze list (wake them up).

```bash
prctrl snooze remove 4821
prctrl snooze remove --pr 4821
prctrl snooze remove 4821,4822,4823
```

### clear

Clear all snoozed PRs.

```bash
prctrl snooze clear
```

### expire

Remove expired snooze entries.

```bash
prctrl snooze expire
```

### extend

Extend snooze duration for already-snoozed PRs.

```bash
prctrl snooze extend 4821 --days 7
prctrl snooze extend --pr 4821 --days 7
prctrl snooze extend 4821,4822,4823 --days 5
```

## Examples

```bash
# Snooze a single PR for 3 days (using positional argument)
prctrl snooze 4821

# Snooze a single PR using --pr flag
prctrl snooze add --pr 4821

# Snooze for 7 days
prctrl snooze 4821 --days 7

# Snooze all PRs from the last 2 weeks (interactive selection)
prctrl snooze add --since-days 14

# List snoozed PRs as JSON (for scripting)
prctrl snooze list --json

# List snoozed PRs with priority scores
prctrl snooze list --priority

# List snoozed PRs filtered by repository
prctrl snooze list --repo myrepo

# List snoozed PRs filtered by author
prctrl snooze list --author johndoe

# Show detailed view filtered by repo
prctrl snooze review --repo myrepo
```
