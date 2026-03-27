# search

**Search pending reviews by title keyword.**

Find specific PRs without scrolling through the full list.

## When to Use

- Remembering: "Was there a PR about auth?"
- Filtering: "Find all security-related PRs"

## Synopsis

```bash
review-dispatcher search [OPTIONS] <KEYWORD>
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `<KEYWORD>` | Keyword to search for | Required |
| `-P, --priority` | Show priority scores | `false` |

## Examples

```bash
review-dispatcher search auth
```
