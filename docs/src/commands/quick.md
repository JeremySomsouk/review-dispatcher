# quick

**Show "quick win" PRs — small, non-draft PRs you can review in 5 minutes.**

Find low-effort reviews that make a big impact.

## When to Use

- Short break: "Got 5 minutes, review something"
- Review streak: "Keep momentum going"

## Synopsis

```bash
review-dispatcher quick [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-l, --max-lines <NUM>` | Maximum total lines for a "quick" PR | `200` |
| `-n, --limit <NUM>` | Maximum number of results | `10` |
| `-P, --priority` | Show priority scores (1-5 stars based on age and size) | `false` |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# Show quick wins (default: ≤200 lines, non-draft)
review-dispatcher quick

# Show tiny PRs only (≤100 lines)
review-dispatcher quick --max-lines 100

# Show quick wins from a specific repo
review-dispatcher quick --repo myservice

# Show quick wins with priority scores
review-dispatcher quick --priority

# Combine filters
review-dispatcher quick --repo api --author johndoe --priority
```
