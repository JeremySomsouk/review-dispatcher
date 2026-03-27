# unassign

**Remove yourself as a reviewer from a PR.**

The counterpart to `assign` — give up review responsibility when you've been assigned by mistake or the author should have asked someone else.

## When to Use

- Wrong assignment: "I was asked to review this by accident"
- Capacity shift: "I'm too busy, someone else should take this"
- Triage cleanup: Clean up your review queue after re-organizing

## Synopsis

```bash
review-dispatcher unassign [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to unassign yourself from | Required |

## Examples

```bash
# Unassign from a specific PR
review-dispatcher unassign 4821

# Interactive selection from pending reviews
review-dispatcher unassign
```

## Tips

- If no PR number is provided, shows your pending reviews and lets you select one interactively
- Use `list` first to see which PRs you're currently assigned to
- After unassigning, the PR author may need to manually request another reviewer
