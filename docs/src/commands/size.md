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
review-dispatcher size [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-f, --filter-size <SIZES>` | Show only specific size(s): XS, S, M, L, XL (comma-separated) | All sizes |
| `-g, --grouped` | Group output by size bucket | `false` |
| `-P, --priority` | Show priority scores (1-5 stars based on age and size) | `false` |
| `--repo <REPO>` | Filter by repository name (partial match, case-insensitive) | All repos |
| `--author <AUTHOR>` | Filter by author username (partial match, case-insensitive) | All authors |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# Show all PRs by size
review-dispatcher size

# Group by size bucket with headers
review-dispatcher size --grouped

# Show only small and medium PRs
review-dispatcher size --filter-size S,M

# Show XS PRs with priority scores
review-dispatcher size --filter-size XS --priority

# Filter by repository
review-dispatcher size --repo myservice

# Filter by author
review-dispatcher size --author johndoe

# JSON output for scripting
review-dispatcher size --json
```
