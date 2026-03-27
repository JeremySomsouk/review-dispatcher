# comment

**Post a comment on a PR directly from the CLI.**

Share feedback, ask questions, or leave notes — all without leaving your terminal.

## When to Use

- Leave feedback: "Minor nit, consider fixing"
- Ask questions: "Can you explain this?"
- Document decisions: "Approved with this note"

## Synopsis

```bash
review-dispatcher comment [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to comment on | Required |
| `-t, --text <TEXT>` | Comment text (supports markdown) | Required |

## Examples

```bash
review-dispatcher comment 4821 --text "Looks good, just a few nits"
```
