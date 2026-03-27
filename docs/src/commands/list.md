# list

List all PRs waiting for your review.

## Synopsis

```bash
review-dispatcher list [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--json` | Output as JSON | false |
| `-s, --since-days <DAYS>` | Only show PRs created since N days ago | - |
| `-P, --priority` | Show priority score (1-5 stars) | false |

## Examples

```bash
# List all pending reviews
review-dispatcher list

# Show with priority scores
review-dispatcher list --priority

# Show only recent PRs (last 7 days)
review-dispatcher list --since-days 7

# JSON output for scripting
review-dispatcher list --json
```

## Output

Displays PR number, title, author, repository, age, and lines changed.

```
🔍 3 pending review(s)
  [1] feat: add CSV export  #4821 (frontend)
  [2] fix: flaky CI        #312 (backend)  
  [3] chore: bump deps     #891 (excluded)
```
