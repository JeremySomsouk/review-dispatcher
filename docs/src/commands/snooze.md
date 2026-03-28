# snooze

**Temporarily hide PRs from the pending list.**

Snooze PRs you're not ready to review yet.

## When to Use

- Context overload: "Not now, remind me later"
- Vacation: "Hide until I return"

## Synopsis

```bash
review-dispatcher snooze [OPTIONS] [PR_NUMBERS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBERS` | PR number(s) to snooze | Required |
| `-d, --days <NUM>` | Days to snooze | `3` |
| `--json` | Output as JSON (useful for scripting) | `false` |
| `-P, --priority` | Show priority scores for listed snoozed PRs | `false` |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | `none` |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | `none` |

## Subcommands

### list

List all currently snoozed PRs.

```bash
review-dispatcher snooze list
review-dispatcher snooze list --repo myrepo
review-dispatcher snooze list --author johndoe
review-dispatcher snooze list --repo myrepo --author johndoe
```

### review

Show detailed view of snoozed PRs with full metadata.

```bash
review-dispatcher snooze review
review-dispatcher snooze review --repo myrepo
```

### add

Add PR(s) to the snooze list.

```bash
review-dispatcher snooze add 4821
review-dispatcher snooze add 4821 --days 7
review-dispatcher snooze add 4821,4822,4823
```

### remove

Remove PR(s) from the snooze list (wake them up).

```bash
review-dispatcher snooze remove 4821
```

### clear

Clear all snoozed PRs.

```bash
review-dispatcher snooze clear
```

### expire

Remove expired snooze entries.

```bash
review-dispatcher snooze expire
```

### extend

Extend snooze duration for already-snoozed PRs.

```bash
review-dispatcher snooze extend 4821 --days 7
```

## Examples

```bash
# Snooze a single PR for 3 days
review-dispatcher snooze 4821

# Snooze for 7 days
review-dispatcher snooze 4821 --days 7

# List snoozed PRs as JSON (for scripting)
review-dispatcher snooze list --json

# List snoozed PRs with priority scores
review-dispatcher snooze list --priority

# List snoozed PRs filtered by repository
review-dispatcher snooze list --repo myrepo

# List snoozed PRs filtered by author
review-dispatcher snooze list --author johndoe

# Show detailed view filtered by repo
review-dispatcher snooze review --repo myrepo
```
