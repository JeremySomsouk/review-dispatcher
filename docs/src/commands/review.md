# review

**Fetch and display a PR diff in the terminal with syntax highlighting.**

Full code review in your terminal — no browser needed.

## When to Use

- Deep dive: "I need to see the actual code"
- Offline review: "No browser, but need to review"

## Synopsis

```bash
review-dispatcher review [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to review | Required |
| `-c, --context <NUM>` | Context lines around changes | `3` |
| `-o, --output <FILE>` | Output diff to file | Terminal |

## Examples

```bash
review-dispatcher review 4821
review-dispatcher review 4821 --context 5
```
