# stats

**Show review statistics: pending count, average wait time, and breakdowns by repository and author.**

Gives you a high-level overview of your review queue at a glance.

## When to Use

- Morning check: "How bad is my backlog?"
- After returning from time off: "How much did I miss?"
- Reporting to team lead: "What's the review load?"

## Synopsis

```bash
review-dispatcher stats [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# See review statistics at a glance
review-dispatcher stats

# Get JSON for dashboards or automation
review-dispatcher stats --json
```

## Output Example

```
📊 Review Statistics
────────────────────────────────────────
  Total pending reviews: 12
  Total lines changed:   +4821 / -1203

  Avg time waiting:      4 days
  Oldest PR:             #4821 (7 days ago)

  By repository:
    frontend: 5
    backend: 4
    shared: 3

  By author:
    alice   ████████
    bob     ██████
    carol   ████
```

## Tips

- Use `--json` for piping into scripts or generating metrics
- The "avg time waiting" metric helps you spot if PRs are piling up
- Author breakdown shows who's waiting most on you
