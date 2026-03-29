# ci

**Show CI/CD pipeline status for pending PRs — GitHub Actions, etc.**

See which PRs are green, failing, or still running.

## When to Use

- Pre-review check: "Is this PR even ready?"
- CI debugging: "Why is this failing?"
- Merge planning: "Which PRs can I merge now?"

## Filter Behavior

Filters (`--repo`, `--author`, `--since-days`) are applied **before** fetching CI status, reducing API calls and improving performance. When a specific PR is targeted via positional argument or `--pr`, filters are skipped to ensure accurate targeting.

## Synopsis

```bash
prctrl ci [OPTIONS]
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
| `-P, --priority` | Show priority scores for each PR (1-5 stars based on age and size) | `false` |
| `--json` | Output as JSON | `false` |

## Examples

```bash
prctrl ci
prctrl ci --failing-only
prctrl ci --passing-only
prctrl ci -f -a
prctrl ci --repo myrepo
prctrl ci --author johndoe
prctrl ci --repo myrepo --failing-only
prctrl ci --pr 123
prctrl ci 123
prctrl ci --since-days 7
prctrl ci --since-days 3 --failing-only
prctrl ci --priority
prctrl ci -f --priority
```
