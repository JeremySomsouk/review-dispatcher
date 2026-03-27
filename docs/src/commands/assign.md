# assign

**Assign yourself as a reviewer on a PR.**

Skip the web UI — claim review responsibility directly from the terminal.

## When to Use

- Quick claim: "I want to review this before anyone else"
- Triage workflow: Pair with `delegate` for AI-assisted assignment

## Synopsis

```bash
review-dispatcher assign [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to assign yourself to | Required |

## Examples

```bash
review-dispatcher assign 4821
```
