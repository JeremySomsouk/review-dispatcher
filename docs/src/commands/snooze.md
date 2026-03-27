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

## Examples

```bash
review-dispatcher snooze 4821
review-dispatcher snooze 4821 --days 7
```
