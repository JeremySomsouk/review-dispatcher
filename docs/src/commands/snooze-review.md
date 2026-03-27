# snooze review

**Show detailed information about your snoozed PRs — author, age, size, and priority.**

When you've snoozed multiple PRs and want to review them in detail before waking them up, `snooze review` provides a richer view than `snooze list`. It shows the full context: who authored each PR, how many lines were changed, when you snoozed it, and the original priority score.

## When to Use

- "I have 5 snoozed PRs — which one should I wake up first?"
- Before unwaking a PR, check its current priority
- Review the context of snoozed PRs without fetching fresh GitHub data
- Plan your review session by understanding what you deferred

## Synopsis

```bash
review-dispatcher snooze review
```

## Options

This command has no additional options — it always shows the full detailed view of all currently snoozed PRs.

## Examples

```bash
# View all snoozed PRs with full details
review-dispatcher snooze review

# See which ones are coming close to expiry
review-dispatcher snooze review
```

## Output

`snooze review` shows for each snoozed PR:

| Field | Description |
|-------|-------------|
| Title & Number | PR title (bold) and number |
| Repository | Which repo the PR is in |
| Snoozed At | When you snoozed it (e.g., "2h ago", "1d ago") |
| Time Remaining | How long until it auto-expires (e.g., "6h left", "2d left") |
| Lines Changed | +additions / -deletions |
| Priority Score | Original star rating when snoozed |
| URL | Direct link to the PR |

## Comparison with `snooze list`

| Feature | `snooze list` | `snooze review` |
|---------|---------------|-----------------|
| Title | ✅ | ✅ |
| Number | ✅ | ✅ |
| Repository | ✅ | ✅ |
| Time Remaining | ✅ | ✅ |
| Author | ❌ | ✅ |
| Lines Changed | ❌ | ✅ |
| Priority Score | ❌ | ✅ |
| URL | ❌ | ✅ |
| Snoozed At | ❌ | ✅ |

## Tips

- Use `snooze review` before `snooze remove` to remember why you snoozed something
- If a snoozed PR's priority score was high (4-5 stars), consider prioritizing it when you wake it
- Snoozed PRs automatically expire and reappear in your pending list

## See Also

- [`snooze list`](./snooze-list.md) — Quick summary view
- [`snooze remove`](./snooze-remove.md) — Wake a PR early
- [`snooze extend`](./snooze-extend.md) — Extend snooze duration
- [`snooze expire`](./snooze-expire.md) — Remove expired entries
