# search

Search pending reviews by title keyword.

## Synopsis

```bash
review-dispatcher search <KEYWORD> [OPTIONS]
```

## Arguments

| Argument | Description |
|----------|-------------|
| `KEYWORD` | Keyword to search for in PR titles |

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-P, --priority` | Show priority scores | false |
| `--json` | Output as JSON | false |

## Examples

```bash
# Search for "security" PRs
review-dispatcher search security

# Search with priority scores
review-dispatcher search "api" --priority
```
