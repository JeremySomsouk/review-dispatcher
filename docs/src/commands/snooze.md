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
```
