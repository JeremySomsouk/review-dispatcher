# unassign

**Remove yourself as a reviewer from a PR.**

The counterpart to `assign` — give up review responsibility when you've been assigned by mistake or the author should have asked someone else.

## When to Use

- Wrong assignment: "I was asked to review this by accident"
- Capacity shift: "I'm too busy, someone else should take this"
- Triage cleanup: Clean up your review queue after re-organizing
- Batch operation: Use `--all` to unassign from all pending reviews at once
- Scripting: Use `--json` for programmatic integrations

## Synopsis

```bash
prctrl unassign [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to unassign yourself from (shorthand for `--pr`) | Required if no `--pr` |
| `-p, --pr` | Global flag: target a specific PR number | - |
| `-a, --all` | Unassign yourself from all pending reviews at once | `false` |
| `-n, --pr-numbers` | PR number(s) to unassign from (comma-separated, e.g. `123,456`) | - |
| `-s, --since-days` | Only show PRs created since this many days ago | - |
| `-n, --dry-run` | Preview what would be unassigned without actually removing | `false` |
| `--json` | Output as JSON for scripting | `false` |
| `-q, --quiet` | Suppress per-PR progress messages (show only summary) | `false` |
| `--repo` | Filter by repository name (partial match, case-insensitive) | - |
| `--author` | Filter by author username (partial match, case-insensitive) | - |
| `-P, --priority` | Show priority scores for each PR (1-5 stars based on age and size) | `false` |

## Examples

```bash
# Unassign from a specific PR
prctrl unassign 4821

# Preview what would be unassigned (dry-run)
prctrl unassign --all --dry-run

# Unassign from all pending reviews at once
prctrl unassign --all

# Unassign from multiple specific PRs
prctrl unassign --pr-numbers 4821,4822,4823

# Interactive selection from pending reviews
prctrl unassign

# Unassign with JSON output (for scripting)
prctrl unassign 4821 --json

# Unassign from all PRs created in the last 3 days
prctrl unassign --all --since-days 3

# Unassign with priority scores shown
prctrl unassign --priority
```

## Tips

- Use `--all` to quickly unassign yourself from ALL pending reviews without prompting
- Use `--pr-numbers` to unassign from multiple specific PRs in one command
- Use `--dry-run` to preview what would be unassigned before making changes
- Parallel requests are used when unassigning from multiple PRs for speed
- If no PR number is provided and `--all` is not used, shows your pending reviews and lets you select interactively

## JSON Output

When `--json` is used, returns an array of results:

```json
[
  {
    "pr_number": 4821,
    "pr_title": "Add user authentication",
    "repo": "myorg/myrepo",
    "url": "https://github.com/myorg/myrepo/pull/4821",
    "success": true,
    "error": null
  }
]
```
