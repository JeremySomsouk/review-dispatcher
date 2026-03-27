# snooze expire

**Remove expired snooze entries and restore PRs to your pending list.**

Automatically clean up snoozed PRs whose snooze duration has elapsed. Expired entries are permanently removed from the snooze list, making those PRs visible again in your pending reviews.

## When to Use

- Routine maintenance: Run periodically to keep your snooze list clean
- After returning from vacation: Clean up old snoozes you forgot about
- As part of a daily/weekly script: Automate cleanup alongside other review tasks

## Synopsis

```bash
review-dispatcher snooze expire
```

## Options

This subcommand has no additional flags.

## Examples

```bash
# Clean up all expired snooze entries
review-dispatcher snooze expire

# Expected output (when there are expired entries):
# 🧹 Cleaned up 3 expired snooze entry(s):
#   ✨ 3 PR(s) have returned to your pending list.
# ✅ 7 snoozed PR(s) remain in the list.

# Expected output (when nothing has expired):
# ✨ No expired snooze entries to clean up.
```

## How It Works

1. Reads the `.snoozed.json` file from your output directory
2. Checks each entry's `snoozed_until` timestamp against the current time
3. Removes entries where the snooze period has passed
4. Saves the updated list back to `.snoozed.json`
5. Reports how many entries were cleaned and how many remain

## Relationship to Other Commands

| Command | Purpose |
|---------|---------|
| `snooze add` | Add PRs to the snooze list |
| `snooze list` | Show currently snoozed PRs with time remaining |
| `snooze remove` | Manually remove specific PRs from snooze |
| `snooze clear` | Remove ALL snoozed PRs at once |
| `snooze expire` | Automatically remove PRs whose snooze has elapsed |

## Tips

- Run `snooze expire` before `list` to see all PRs including recently snoozed ones that have expired
- For automation, add `snooze expire` to your daily review script before fetching pending reviews
- Snoozed PRs are hidden from `list` but don't automatically reappear — use `expire` to restore them