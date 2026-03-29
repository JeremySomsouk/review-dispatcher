# quick

**Show "quick win" PRs — small, non-draft PRs you can review in 5 minutes.**

Find low-effort reviews that make a big impact.

## When to Use

- Short break: "Got 5 minutes, review something"
- Review streak: "Keep momentum going"

## Synopsis

```bash
prctrl quick [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-l, --max-lines <NUM>` | Maximum total lines for a "quick" PR | `200` |
| `-n, --limit <NUM>` | Maximum number of results | `10` |
| `-P, --priority` | Show priority scores (1-5 stars based on age and size) | `false` |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | |
| `-s, --since-days <NUM>` | Only show PRs created since this many days ago | |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# Show quick wins (default: ≤200 lines, non-draft)
prctrl quick

# Show tiny PRs only (≤100 lines)
prctrl quick --max-lines 100

# Show quick wins from a specific repo
prctrl quick --repo myservice

# Show quick wins with priority scores
prctrl quick --priority

# Only show recent quick wins (last 7 days)
prctrl quick --since-days 7

# Combine filters
prctrl quick --repo api --author johndoe --priority --since-days 14
```
