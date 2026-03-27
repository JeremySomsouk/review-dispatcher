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
| `-f, --failing` | Only show PRs with failing checks | `false` |
| `-p, --passing` | Only show PRs with passing checks | `false` |
| `-a, --all` | Show CI status for all pending reviews | `false` |
| `-n, --prs <NUMS>` | PR number(s) to check (comma-separated) | - |
| `--json` | Output as JSON | `false` |

## Examples

```bash
review-dispatcher ci
review-dispatcher ci --failing
```
