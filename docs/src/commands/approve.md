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
review-dispatcher approve [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-p, --pr <NUM>` | PR number to approve | - |
| `-a, --all` | Approve all pending reviews at once | `false` |
| `-n, --pr-numbers` | PR number(s) to approve (comma-separated, e.g. `123,456`) | - |
| `-m, --message <TEXT>` | Approval comment (optional, default: "LGTM!") | `LGTM!` |
| `-s, --since-days <DAYS>` | Only approve PRs created since this many days ago | - |
| `-P, --priority` | Show priority scores for each PR (1-5 stars based on age and size) | `false` |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | - |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | - |
| `--json` | Output as JSON (useful for scripting) | `false` |

## Examples

```bash
# Approve a specific PR with default message
review-dispatcher approve --pr 4821

# Approve with a custom comment
review-dispatcher approve --pr 4821 -m "LGTM! Nice work on the tests."

# Approve without comment
review-dispatcher approve --pr 4821 -m ""

# Approve all pending reviews at once
review-dispatcher approve --all

# Approve all PRs from a specific author
review-dispatcher approve --all --author johndoe

# Approve all PRs from a specific repository
review-dispatcher approve --all --repo myservice

# Approve PRs from the last 3 days only
review-dispatcher approve --all --since-days 3

# Approve with priority scores shown
review-dispatcher approve --all --priority

# Approve multiple specific PRs
review-dispatcher approve --pr-numbers 4821,4822,4823

# Approve with JSON output (for scripting)
review-dispatcher approve --pr 4821 --json
```

## Tips

- Use `--all` to quickly approve ALL pending reviews at once
- Use `--author` and `--repo` filters to narrow down which reviews to approve
- Use `--priority` to see priority scores when selecting reviews interactively
- Use `--pr-numbers` to approve multiple specific PRs in one command
- Parallel requests are used when approving multiple PRs for speed
- If no PR number is provided and `--all` is not used, shows your pending reviews and lets you select interactively
- Requires PR to already be reviewed (or at least have the PR in a reviewable state)
