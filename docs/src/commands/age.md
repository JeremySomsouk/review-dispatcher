# age

**Categorize pending PRs by age — new, aging, stale, or overdue.**

Visual buckets help you spot neglected PRs at a glance.

## When to Use

- Morning overview: "What's new vs. what's been waiting?"
- Sprint planning: "What might become overdue?"

## Age Buckets

| Bucket | Age |
|--------|-----|
| 🆕 New | 0-2 days |
| ⏳ Aging | 3-5 days |
| ⚠️ Stale | 6-9 days |
| 🔴 Overdue | 10+ days |

## Synopsis

```bash
review-dispatcher age [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-n, --min-days <DAYS>` | Show only PRs newer than N days | - |
| `-x, --older-than <DAYS>` | Show only PRs older than N days | - |
| `-g, --grouped` | Group output by age bucket | `false` |
| `--json` | Output as JSON | `false` |

## Examples

```bash
review-dispatcher age
review-dispatcher age --grouped
```
