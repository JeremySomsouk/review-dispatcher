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
review-dispatcher comment [OPTIONS] [PR_NUMBER]
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
| `--repo` | Filter by repository name (partial match, case-insensitive) | - |
| `--author` | Filter by author username (partial match, case-insensitive) | - |

## Examples

```bash
# Comment on a specific PR
review-dispatcher comment 4821 --text "Looks good, just a few nits"

# Preview what would be commented (dry-run)
review-dispatcher comment --all --text "Please address feedback" --dry-run

# Comment on all pending reviews at once
review-dispatcher comment --all --text "Please address feedback before merging"

# Comment on multiple specific PRs
review-dispatcher comment --pr-numbers 4821,4822 --text "LGTM!"

# With JSON output (for scripting)
review-dispatcher comment 4821 --text "LGTM!" --json
```

## Tips

- Use `--all` to quickly post the same comment to ALL pending reviews at once
- Use `--pr-numbers` to comment on multiple specific PRs in one command
- Use `--dry-run` to preview what would be commented before posting
- Parallel requests are used when commenting on multiple PRs for speed
- If no PR number is provided and `--all` is not used, shows your pending reviews and lets you select interactively
