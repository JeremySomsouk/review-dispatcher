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
| XL | 1000+ |

## Synopsis

```bash
review-dispatcher size [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-g, --grouped` | Group output by size bucket | `false` |

## Examples

```bash
review-dispatcher size
review-dispatcher size --grouped
```
