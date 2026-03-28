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
| `-p, --passing-only` | Only show PRs with passing checks | `false` |
| `-a, --all` | Show CI status for all pending reviews | `false` |
| `-n, --pr-numbers <NUMS>` | PR number(s) to check (comma-separated) | - |
| `-r, --repo <REPO>` | Filter by repository name (partial match) | - |
| `--author <AUTHOR>` | Filter by author username (partial match) | - |
| `--json` | Output as JSON | `false` |

Note: `-p` is the short form for `--passing-only` in this command. Use `--pr` (global flag) to target a specific PR number.

## Examples

```bash
review-dispatcher ci
review-dispatcher ci --failing-only
review-dispatcher ci --passing-only
review-dispatcher ci -f -a
review-dispatcher ci --repo myrepo
review-dispatcher ci --author johndoe
review-dispatcher ci --repo myrepo --failing-only
```
