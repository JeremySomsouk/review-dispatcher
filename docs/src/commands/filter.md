# filter

Filter pending reviews by various criteria.

## Synopsis

```bash
review-dispatcher filter [OPTIONS]
```

## Options

| Flag | Description |
|------|-------------|
| `--repo <NAME>` | Filter by repository (partial match) |
| `--author <NAME>` | Filter by author (partial match) |
| `--min-size <LINES>` | Minimum total lines changed |
| `--max-size <LINES>` | Maximum total lines changed |
| `--min-age <DAYS>` | Minimum age in days |
| `--max-age <DAYS>` | Maximum age in days |
| `--drafts-only` | Show only draft PRs |
| `--no-drafts` | Hide draft PRs |
| `-P, --priority` | Show priority scores |
| `--json` | Output as JSON |

## Examples

```bash
# Filter by repository
review-dispatcher filter --repo frontend

# Filter by author
review-dispatcher filter --author alice

# Large PRs only (500+ lines)
review-dispatcher filter --min-size 500

# Combine filters
review-dispatcher filter --repo api --min-age 7 --priority
```
