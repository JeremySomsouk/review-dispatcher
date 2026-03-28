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
| `-P, --passing-only` | Only show PRs with passing checks | `false` |
| `-a, --all` | Show CI status for all pending reviews | `false` |
| `-n, --pr-numbers <NUMS>` | PR number(s) to check (comma-separated) | - |
| `--json` | Output as JSON | `false` |

Note: `-p` is reserved globally for `--pr` (target specific PR). Use `-P` (uppercase) for `--passing-only`.

## Examples

```bash
review-dispatcher ci
review-dispatcher ci --failing-only
review-dispatcher ci --passing-only
review-dispatcher ci -f -a
```
