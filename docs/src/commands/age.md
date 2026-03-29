# age

**Categorize pending PRs by age — new, aging, stale, or overdue.**

Visual buckets help you spot neglected PRs at a glance.

## When to Use

- Morning overview: "What's new vs. what's been waiting?"
- Sprint planning: "What might become overdue?"

## Age Buckets

| Bucket | Age |
|--------|-----|
| 🆕 New | 0-1 days |
| 🌱 Fresh | 2-3 days |
| ⏳ Aging | 4-7 days |
| 🔥 Stale | 8-14 days |
| 💀 Overdue | 15+ days |

## Synopsis

```bash
prctrl age [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-n, --min-days <DAYS>` | Show only PRs newer than N days | - |
| `-x, --older-than <DAYS>` | Show only PRs older than N days | - |
| `-g, --grouped` | Group output by age bucket | `false` |
| `-P, --priority` | Show priority scores for each PR (1-5 stars) | `false` |
| `--repo <REPO>` | Filter by repository name (partial match) | - |
| `--author <AUTHOR>` | Filter by author username (partial match) | - |
| `--json` | Output as JSON | `false` |

## Examples

```bash
# Basic age overview
prctrl age

# Grouped view by bucket
prctrl age --grouped

# Show priority scores
prctrl age --priority

# Focus on older PRs
prctrl age --older-than 7
prctrl age --older-than 14 --grouped

# Filter by repo or author
prctrl age --repo myservice
prctrl age --author john
```
