# approve

**Approve a PR directly from the terminal.**

No more switching to GitHub UI for simple approvals. Approve and add a comment in one command.

## When to Use

- Code looks good after review
- Small PR you trust the author on
- Quick approval to unblock CI
- Batch approval: Use `--all` to approve all pending reviews at once

## Synopsis

```bash
prctrl approve [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-p, --pr <NUM>` | PR number to approve | - |
| `-a, --all` | Approve all pending reviews at once | `false` |
| `--pr-numbers` | PR number(s) to approve (comma-separated, e.g. `123,456`) | - |
| `-m, --message <TEXT>` | Approval comment (optional, default: "LGTM!") | `LGTM!` |
| `-s, --since-days <DAYS>` | Only approve PRs created since this many days ago | - |
| `-n, --dry-run` | Preview what would be approved without actually approving | `false` |
| `-P, --priority` | Show priority scores for each PR (1-5 stars based on age and size) | `false` |
| `-q, --quiet` | Suppress per-PR progress messages (show only summary) | `false` |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | - |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | - |
| `--json` | Output as JSON (useful for scripting) | `false` |

## Examples

```bash
# Approve a specific PR with default message
prctrl approve --pr 4821

# Approve with a custom comment
prctrl approve --pr 4821 -m "LGTM! Nice work on the tests."

# Approve without comment
prctrl approve --pr 4821 -m ""

# Preview what would be approved (dry-run)
prctrl approve --pr 4821 -n

# Preview what would be approved for all pending
prctrl approve --all -n

# Approve all pending reviews at once
prctrl approve --all

# Approve all PRs from a specific author
prctrl approve --all --author johndoe

# Approve all PRs from a specific repository
prctrl approve --all --repo myservice

# Approve PRs from the last 3 days only
prctrl approve --all --since-days 3

# Approve with priority scores shown
prctrl approve --all --priority

# Approve multiple specific PRs
prctrl approve --pr-numbers 4821,4822,4823

# Approve with JSON output (for scripting)
prctrl approve --pr 4821 --json
```

## Tips

- Use `--dry-run` (`-n`) to preview what would be approved before actually approving
- When `--pr` matches a PR in multiple repos, you'll be asked to choose which one
- **Confirmation is required** before approving (unless using `--dry-run`)

## Disambiguation

If your organization has the same PR number across different repositories, using `--pr` will show a selection menu:

```
📋 PR #4821 found in multiple repos:

  1. frontend / #4821 feat: add dark mode
  2. backend / #4821 fix: login timeout

Select repo (q to quit):
```

This prevents accidentally approving the wrong PR.
- Use `--all` to quickly approve ALL pending reviews at once
- Use `--author` and `--repo` filters to narrow down which reviews to approve
- Use `--priority` to see priority scores when selecting reviews interactively
- Use `--pr-numbers` to approve multiple specific PRs in one command
- Parallel requests are used when approving multiple PRs for speed
- If no PR number is provided and `--all` is not used, shows your pending reviews and lets you select interactively
- Requires PR to already be reviewed (or at least have the PR in a reviewable state)
