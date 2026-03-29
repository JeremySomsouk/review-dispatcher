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
review-dispatcher unassign [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to unassign yourself from (shorthand for `--pr`) | Required if no `--pr` |
| `-p, --pr` | Global flag: target a specific PR number | - |
| `-a, --all` | Unassign yourself from all pending reviews at once | `false` |
| `-n, --pr-numbers` | PR number(s) to unassign from (comma-separated, e.g. `123,456`) | - |
| `-s, --since-days` | Only show PRs created since this many days ago | - |
| `--json` | Output as JSON for scripting | `false` |
| `--repo` | Filter by repository name (partial match, case-insensitive) | - |
| `--author` | Filter by author username (partial match, case-insensitive) | - |

## Examples

```bash
# Unassign from a specific PR
review-dispatcher unassign 4821

# Unassign from all pending reviews at once
review-dispatcher unassign --all

# Unassign from multiple specific PRs
review-dispatcher unassign --pr-numbers 4821,4822,4823

# Interactive selection from pending reviews
review-dispatcher unassign

# Unassign with JSON output (for scripting)
review-dispatcher unassign 4821 --json

# Unassign from all PRs created in the last 3 days
review-dispatcher unassign --all --since-days 3
```

## Tips

- Use `--all` to quickly unassign yourself from ALL pending reviews without prompting
- Use `--pr-numbers` to unassign from multiple specific PRs in one command
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
