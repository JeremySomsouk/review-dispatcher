# stats

**Show review statistics: pending count, average wait time, and breakdowns by repository and author.**

Gives you a high-level overview of your review queue at a glance.

## When to Use

- Morning overview: "How bad is it?"
- Sprint planning: "What's the review load?"

## Synopsis

```bash
review-dispatcher stats [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--json` | Output as JSON | `false` |
| `-p, --pr <NUMBER>` | Filter to a specific PR number | - |
| `--repo <NAME>` | Filter by repository (partial match, case-insensitive) | - |
| `--author <NAME>` | Filter by author (partial match, case-insensitive) | - |

## Examples

```bash
# See all review statistics
review-dispatcher stats

# Stats as JSON for scripting
review-dispatcher stats --json

# Stats for a specific repo only
review-dispatcher stats --repo frontend

# Stats filtered by author
review-dispatcher stats --author alice

# Stats for a specific PR
review-dispatcher stats --pr 4821
```
