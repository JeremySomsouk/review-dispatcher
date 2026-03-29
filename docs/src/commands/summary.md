# summary

**Show a quick one-line summary of pending reviews.**

The fastest way to get an at-a-glance view of your review queue.

## When to Use

- Slack check: "How backed up am I?"
- Shell prompt: "Drop it in your terminal status bar"

## Synopsis

```bash
review-dispatcher summary [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--json` | Output as JSON | `false` |
| `--repo` | Filter by repository name (partial match, case-insensitive) | |
| `--author` | Filter by author username (partial match, case-insensitive) | |
| `--since-days`, `-s` | Only show PRs created since this many days ago | |

## Examples

```bash
review-dispatcher summary
review-dispatcher summary --json
review-dispatcher summary --repo myrepo
review-dispatcher summary --author johndoe
review-dispatcher summary --since-days 7
```

## Output Example

```
🔍 12 pending | Oldest: 8 days | +892 lines | 3 urgent, 5 normal, 4 low
```
