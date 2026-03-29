# assign

**Assign yourself as a reviewer on a PR.**

Skip the web UI — claim review responsibility directly from the terminal.

## When to Use

- Quick claim: "I want to review this before anyone else"
- Triage workflow: Pair with `delegate` for AI-assisted assignment
- Scripting: Use `--json` for programmatic integrations
- Batch operation: Use `--all` to assign to all pending reviews at once

## Synopsis

```bash
review-dispatcher assign [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to assign yourself to (shorthand for `--pr`) | Required if no `--pr` |
| `-p, --pr` | Global flag: target a specific PR number | - |
| `-a, --all` | Assign yourself to all pending reviews at once | `false` |
| `-n, --pr-numbers` | PR number(s) to assign (comma-separated, e.g. `123,456`) | - |
| `-s, --since-days` | Only show PRs created since this many days ago | - |
| `-n, --dry-run` | Preview what would be assigned without actually assigning | `false` |
| `--json` | Output as JSON for scripting | `false` |
| `-q, --quiet` | Suppress per-PR progress messages (show only summary) | `false` |
| `--repo` | Filter by repository name (partial match, case-insensitive) | - |
| `--author` | Filter by author username (partial match, case-insensitive) | - |
| `-P, --priority` | Show priority scores for each PR (1-5 stars based on age and size) | `false` |

## Examples

```bash
# Assign to a specific PR
review-dispatcher assign 4821

# Preview what would be assigned (dry-run)
review-dispatcher assign --all --dry-run

# Assign using global --pr flag
review-dispatcher --pr 4821 assign

# Assign to all pending reviews at once
review-dispatcher assign --all

# Assign to multiple specific PRs
review-dispatcher assign --pr-numbers 4821,4822,4823

# Assign with JSON output (for scripting)
review-dispatcher assign 4821 --json

# Assign to all PRs created in the last 3 days
review-dispatcher assign --all --since-days 3

# Assign with priority scores shown (to pick highest priority PRs)
review-dispatcher assign --priority
```

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

## Tips

- Use `--all` to quickly assign yourself to ALL pending reviews without prompting
- Use `--pr-numbers` to assign to multiple specific PRs in one command
- Use `--dry-run` to preview what would be assigned before making changes
- Parallel requests are used when assigning to multiple PRs for speed
