# search

**Search pending reviews by title keyword, optionally filtered by repo or author.**

Find specific PRs without scrolling through the full list.

## When to Use

- Remembering: "Was there a PR about auth?"
- Filtering: "Find all security-related PRs from a specific author"

## Synopsis

```bash
review-dispatcher search [OPTIONS] <KEYWORD>
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `<KEYWORD>` | Keyword to search for in PR titles | Required |
| `--repo` | Filter by repository name (partial match, case-insensitive) | None |
| `--author` | Filter by author username (partial match, case-insensitive) | None |
| `-P, --priority` | Show priority scores | `false` |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# Basic search
review-dispatcher search auth

# Search within a specific repo
review-dispatcher search auth --repo myservice

# Search for a PR by a specific author
review-dispatcher search feature --author johndoe

# Combine filters
review-dispatcher search fix --repo api --author alice --priority

# JSON output for scripting
review-dispatcher search auth --json
```
