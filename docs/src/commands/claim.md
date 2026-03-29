# claim

**Claim multiple PRs for review at once.**

Sign up for review responsibility without clicking through the web UI.

## When to Use

- Sprint start: "I'll take these three"
- Batch workflow: "Claim, review, repeat"

## Synopsis

```bash
prctrl claim [OPTIONS] [PR_NUMBERS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-a, --all` | Claim all matching PRs (use with `--repo`/`--author` filters) | `false` |
| `-n, --dry-run` | Preview what would be claimed without taking action | `false` |
| `-P, --priority` | Show priority scores for each PR | `false` |
| `-s, --since-days` | Only show PRs created since this many days ago | - |
| `--repo` | Filter by repository name (partial match) | - |
| `--author` | Filter by author username (partial match) | - |
| `--json` | Output results as JSON | `false` |
| `-q, --quiet` | Suppress per-PR progress messages (show only summary) | `false` |
| `PR_NUMBERS` | PR number(s) to claim (comma-separated) | - |

## Examples

```bash
# Claim specific PRs by number
prctrl claim 4821,3156,2890

# Claim all pending reviews
prctrl claim --all

# Preview what would be claimed (dry-run)
prctrl claim --all --dry-run

# Claim all PRs from a specific repo
prctrl claim --all --repo myservice

# Claim all PRs from a specific author with priority scores
prctrl claim --all --author johndoe --priority

# Claim all PRs from the last 7 days only
prctrl claim --all --since-days 7

# Claim with JSON output for scripting
prctrl claim --all --json
```
