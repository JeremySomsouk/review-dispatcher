# size

**Categorize pending PRs by size — XS, S, M, L, XL.**

Estimate review effort before you start.

## Size Buckets

| Size | Lines Changed |
|------|---------------|
| XS | 1-50 |
| S | 51-200 |
| M | 201-500 |
| L | 501-1000 |
| XL | 1001+ |

## Synopsis

```bash
prctrl size [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-f, --filter-size <SIZES>` | Show only specific size(s): XS, S, M, L, XL (comma-separated) | All sizes |
| `-g, --grouped` | Group output by size bucket | `false` |
| `-P, --priority` | Show priority scores (1-5 stars based on age and size) | `false` |
| `--repo <REPO>` | Filter by repository name (partial match, case-insensitive) | All repos |
| `--author <AUTHOR>` | Filter by author username (partial match, case-insensitive) | All authors |
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | All PRs |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# Show all PRs by size
prctrl size

# Group by size bucket with headers
prctrl size --grouped

# Show only small and medium PRs
prctrl size --filter-size S,M

# Show XS PRs with priority scores
prctrl size --filter-size XS --priority

# Filter by repository
prctrl size --repo myservice

# Filter by author
prctrl size --author johndoe

# Show only recent PRs (last 7 days)
prctrl size --since-days 7

# JSON output for scripting
prctrl size --json
```
