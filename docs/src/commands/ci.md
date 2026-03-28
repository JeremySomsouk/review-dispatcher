# ci

**Show CI/CD pipeline status for pending PRs — GitHub Actions, etc.**

See which PRs are green, failing, or still running.

## When to Use

- Pre-review check: "Is this PR even ready?"
- CI debugging: "Why is this failing?"
- Merge planning: "Which PRs can I merge now?"

## Synopsis

```bash
review-dispatcher ci [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-f, --failing-only` | Only show PRs with failing checks | `false` |
| `--passing-only` | Only show PRs with passing checks | `false` |
| `-a, --all` | Show CI status for all pending reviews | `false` |
| `-n, --pr-numbers <NUMS>` | PR number(s) to check (comma-separated) | - |
| `PR_NUMBER` | PR number to check CI for (shorthand for --pr) | - |
| `-p, --pr <PR>` | Show CI status for specific PR (shorthand for --pr) | - |
| `-r, --repo <REPO>` | Filter by repository name (partial match) | - |
| `--author <AUTHOR>` | Filter by author username (partial match) | - |
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | - |
| `--json` | Output as JSON | `false` |

## Examples

```bash
review-dispatcher ci
review-dispatcher ci --failing-only
review-dispatcher ci --passing-only
review-dispatcher ci -f -a
review-dispatcher ci --repo myrepo
review-dispatcher ci --author johndoe
review-dispatcher ci --repo myrepo --failing-only
review-dispatcher ci --pr 123
review-dispatcher ci 123
review-dispatcher ci --since-days 7
review-dispatcher ci --since-days 3 --failing-only
```
