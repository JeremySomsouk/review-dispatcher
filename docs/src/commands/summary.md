# summary

**Show a quick one-line summary of pending reviews.**

The fastest way to get an at-a-glance view of your review queue.

## When to Use

- Slack check: "How backed up am I?"
- Shell prompt: "Drop it in your terminal status bar"

## Synopsis

```bash
review-dispatcher summary [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--json` | Output as JSON | `false` |
| `--repo` | Filter by repository name (partial match, case-insensitive) | |
| `--author` | Filter by author username (partial match, case-insensitive) | |
| `--since-days`, `-s` | Only show PRs created since this many days ago | |
| `--priority`, `-P` | Show detailed priority breakdown (1-5 stars) | `false` |

## Examples

```bash
review-dispatcher summary
review-dispatcher summary --json
review-dispatcher summary --repo myrepo
review-dispatcher summary --author johndoe
review-dispatcher summary --since-days 7
review-dispatcher summary --priority
```

## Output Example

```
📋 12 PRs  ⏱️ oldest: 3d  +892/-234 lines  [🔥1/⚡2/📅5/💤4]  (2 draft)
   📁 repo1: 5 • repo2: 4 • repo3: 3

  ⭐ Priority breakdown:
    ⭐⭐⭐⭐⭐  1 PR(s)  •  oldest: 5 days  •  +450/-120 lines
    ⭐⭐⭐⭐   2 PR(s)  •  oldest: 3 days  •  +200/-80 lines
    ⭐⭐⭐    5 PR(s)  •  oldest: 2 days  •  +150/-30 lines
    ⭐⭐     3 PR(s)  •  oldest: 1 day  •  +60/-15 lines
    ⭐      1 PR(s)  •  oldest: today  •  +32/-9 lines

  🚨 Most Urgent:
    Fix authentication bug  #123  ⭐⭐⭐⭐⭐
    👤 johndoe  •  📦 570 lines  •  5 old  •  myorg/myrepo
    🔗 https://github.com/myorg/myrepo/pull/123
```
