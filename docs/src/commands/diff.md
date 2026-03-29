# diff

Show detailed diff/stats for one or more PRs directly in the terminal.

## Synopsis

```bash
review-dispatcher diff [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `<PR_NUMBER>` | Target a specific PR by number (positional) | Interactive selection |
| `--pr, -p <NUMBER>` | Target a specific PR by number (shorthand) | Interactive selection |
| `--pr-numbers <NUMBERS>` | Target multiple PRs by number (comma-separated) | None |
| `--all, -a` | Show diff/stats for all pending reviews without prompting | `false` |
| `--json` | Output as JSON for scripting | `false` |
| `--priority, -P` | Show priority score (1-5 stars based on age and size) | `false` |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | None |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | None |
| `--since-days, -s <DAYS>` | Only show PRs created since this many days ago | None |

## Examples

```bash
# Show diff for a specific PR (positional)
review-dispatcher diff 4821

# Show diff for a specific PR (shorthand)
review-dispatcher diff --pr 4821
review-dispatcher diff -p 4821

# Show diffs for multiple PRs at once
review-dispatcher diff --pr-numbers 4821,4822,4823

# Interactive mode (select from pending reviews)
review-dispatcher diff

# Show diff/stats for ALL pending reviews at once
review-dispatcher diff --all

# Show diffs for all PRs in a specific repo
review-dispatcher diff --all --repo frontend

# Filter to specific author before selecting
review-dispatcher diff --author sarah_dev

# Show only recent PRs (last 7 days)
review-dispatcher diff --all --since-days 7

# Combine filters
review-dispatcher diff --all --repo frontend --since-days 14

# JSON output for scripting
review-dispatcher diff --pr 4821 --json

# Show priority scores
review-dispatcher diff --pr 4821 --priority
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
