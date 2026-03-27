# size

**Categorize pending PRs by size (XS/S/M/L/XL) with statistics and visual breakdown.**

Shows how much code is changing in each PR, helping you identify quick wins (small PRs) and plan time for large PRs.

## Size Categories

| Category | Lines Changed | Emoji |
|----------|---------------|-------|
| **XS** | 0-30 | ⚖️ |
| **S** | 31-100 | 🔬 |
| **M** | 101-300 | 📊 |
| **L** | 301-800 | 📈 |
| **XL** | 801+ | 🚀 |

## When to Use

- Find "quick win" PRs you can review in 10 minutes
- Identify massive PRs that need dedicated time blocks
- Plan your review session by workload
- See the distribution of PR sizes across your queue

## Synopsis

```bash
review-dispatcher size [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-s, --filter-size <SIZES>` | Show only specific sizes (XS,S,M,L,XL, comma-separated) | All |
| `-g, --grouped` | Group output by size bucket instead of flat list | `false` |
| `-P, --priority` | Show priority scores for each PR | `false` |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# See all pending PRs sorted by size (smallest first)
review-dispatcher size

# See PRs organized by size bucket
review-dispatcher size --grouped

# Find only quick wins (XS and S PRs)
review-dispatcher size --filter-size XS,S

# Find large PRs that need more time
review-dispatcher size --filter-size L,XL

# See size breakdown with priority scores
review-dispatcher size --grouped --priority

# Get JSON for dashboards/scripts
review-dispatcher size --json | jq '.[] | select(.label == "XL")'
```

## Tips

- Use `--filter-size XS,S` to find quick wins before meetings
- Use `--filter-size XL` to identify PRs that need a dedicated 30+ min review block
- Combine with `--priority` to find small but urgent PRs
- XS PRs (under 30 lines) are often hotfixes or small improvements
