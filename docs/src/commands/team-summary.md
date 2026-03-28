# team-summary

**Show team review summary — how many PRs each crew member has waiting.**

See the review load across your team.

## When to Use

- Team standup: "Who has the most reviews?"
- Load balancing: "Can someone help?"

## Synopsis

```bash
review-dispatcher team-summary [--json]
```

## Options

- `--json` — Output as JSON for scripting

## Examples

```bash
# Human-readable output
review-dispatcher team-summary

# JSON output for scripting
review-dispatcher team-summary --json
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
