# diff

Show detailed diff/stats for a specific PR directly in the terminal.

## Synopsis

```bash
review-dispatcher diff [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--pr, -p <NUMBER>` | Target a specific PR by number | Interactive selection |
| `--json` | Output as JSON for scripting | `false` |
| `--priority, -P` | Show priority score (1-5 stars based on age and size) | `false` |

## Examples

```bash
# Show diff for a specific PR
review-dispatcher diff --pr 4821

# Interactive mode (select from pending reviews)
review-dispatcher diff

# JSON output for scripting
review-dispatcher diff --pr 4821 --json
```

## Output

Displays comprehensive PR information including:

- **Title & Number**: PR title with number
- **Author & Age**: Who opened it and when
- **Branch**: Source branch name
- **Status**: DRAFT or READY status
- **Lines**: Additions and deletions
- **Size Category**: XS (<50), S (50-200), M (200-500), L (500-1000), XL (1000+)
- **Age Category**: 🔥 HOT (today), ⚡ FRESH (1-2d), 📅 WEEK OLD (3-7d), ⚠️ STALE (8-14d), 🚨 OLD (15d+)
- **Priority Score**: 1-5 star rating based on urgency (only when `--priority` flag is used)
- **Repository**: Full repo name
- **URL**: Direct link to the PR

## JSON Output

When `--json` is used, outputs a single JSON object per PR:

```json
{
  "pr_number": 4821,
  "pr_title": "feat: add CSV export",
  "repo": "myorg/frontend",
  "author": "sarah_dev",
  "branch": "feature/export",
  "url": "https://github.com/myorg/frontend/pull/4821",
  "age_days": 5,
  "age_category": "WEEK_OLD",
  "size_lines": 277,
  "size_category": "M",
  "additions": 245,
  "deletions": 32,
  "draft": false,
  "priority_score": 3
}
```
