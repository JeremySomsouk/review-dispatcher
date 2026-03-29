# snooze extend

**Extend the snooze duration for already-snoozed PRs.**

Pick up where you left off — push a PR's snooze further into the future without removing and re-adding it.

## When to Use

- "I still can't review this, extend it"
- Deadline approaching: "Push it back another few days"
- Recurring PRs that keep appearing and need continuous snoozing

## Synopsis

```bash
prctrl snooze extend [OPTIONS] [PR_NUMBERS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBERS` | PR number(s) to extend (comma-separated) | Interactive selection |
| `-d, --days <NUM>` | New snooze duration in days | `3` |

## Examples

```bash
# Extend a single PR by 3 days (default)
prctrl snooze extend 4821

# Extend multiple PRs by 7 days
prctrl snooze extend 4821,4519 --days 7

# Interactive: select from currently snoozed PRs
prctrl snooze extend
```

## Output Example

```
✅ Extended 2 PR(s) until 2026-04-03 (7 days)
```

```
📋 Currently snoozed PRs (select to extend):

──────────────────────────────────────────────────
  [1] Fix auth token refresh  #4821 (myorg/frontend) - 2h left
  [2] Update dependencies    #4519 (myorg/backend)  - 18h left
──────────────────────────────────────────────────

 Select PRs to extend [e.g. 1,3 or 1-3 or 'all'] (q to quit): 1,2

✅ Extended 2 PR(s) until 2026-04-03 (3 days)
```

## How It Works

1. Loads the `.snoozed.json` file from your output directory
2. Finds PRs that are currently snoozed
3. Updates their `snoozed_until` timestamp to a new time in the future
4. Saves the updated list back to `.snoozed.json`

## Relationship to Other Commands

| Command | Purpose |
|---------|---------|
| `snooze add` | Add new PRs to the snooze list |
| `snooze list` | Show currently snoozed PRs with time remaining |
| `snooze remove` | Wake a PR early (remove from snooze list) |
| `snooze clear` | Remove ALL snoozed PRs at once |
| `snooze expire` | Automatically remove PRs whose snooze has elapsed |
| `snooze extend` | Extend the snooze duration for already-snoozed PRs |

## Tips

- Use `snooze list` first to see which PRs are currently snoozed and how much time remains
- If you try to extend a PR that isn't in the snooze list, you'll get a warning to use `snooze add` first
- The `--days` flag sets the **new** total duration from now, not additional time
