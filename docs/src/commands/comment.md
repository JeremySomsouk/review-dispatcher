# comment

**Post a comment on a PR directly from the CLI.**

Share feedback, ask questions, or leave notes — all without leaving your terminal.

## When to Use

- Leave feedback: "Minor nit, consider fixing"
- Ask questions: "Can you explain this?"
- Document decisions: "Approved with this note"
- Batch comment: Use `--all` to post the same comment to all pending reviews

## Synopsis

```bash
prctrl comment [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to comment on | Required if no `--pr` |
| `-t, --text <TEXT>` | Comment text (supports markdown) | Required |
| `-a, --all` | Post comment to all pending reviews at once | `false` |
| `--pr-numbers` | PR number(s) to comment on (comma-separated, e.g. `123,456`) | - |
| `-p, --pr` | Global flag: target a specific PR number | - |
| `-n, --dry-run` | Preview what would be commented without actually posting | `false` |
| `--json` | Output as JSON (useful for scripting) |
| `-q, --quiet` | Suppress per-PR progress messages (show only summary) | `false` |
| `-s, --since-days` | Only show PRs created since this many days ago | - |
| `--repo` | Filter by repository name (partial match, case-insensitive) | - |
| `--author` | Filter by author username (partial match, case-insensitive) | - |
| `-P, --priority` | Show priority scores for each PR (1-5 stars based on age and size) | `false` |

## Examples

```bash
# Comment on a specific PR
prctrl comment 4821 --text "Looks good, just a few nits"

# Preview what would be commented (dry-run)
prctrl comment --all --text "Please address feedback" --dry-run

# Comment on all pending reviews at once
prctrl comment --all --text "Please address feedback before merging"

# Comment on multiple specific PRs
prctrl comment --pr-numbers 4821,4822 --text "LGTM!"

# Comment on recent PRs only (last 7 days)
prctrl comment --all --since-days 7 --text "Great work!"

# Comment with priority scores shown (to pick which PRs to comment on)
prctrl comment --priority --text "Please review"
```

## Tips

- Use `--all` to quickly post the same comment to ALL pending reviews at once
- Use `--pr-numbers` to comment on multiple specific PRs in one command
- Use `--dry-run` to preview what would be commented before posting
- Parallel requests are used when commenting on multiple PRs for speed
- If no PR number is provided and `--all` is not used, shows your pending reviews and lets you select interactively
