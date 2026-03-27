# assign

**Assign yourself as a reviewer on a PR.**

Skip the web UI — claim review responsibility directly from the terminal.

## When to Use

- Quick claim: "I want to review this before anyone else"
- Triage workflow: Pair with `delegate` for AI-assisted assignment
- Scripting: Use `--json` for programmatic integrations

## Synopsis

```bash
review-dispatcher assign [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to assign yourself to (shorthand for `--pr`) | Required if no `--pr` |
| `-p, --pr` | Global flag: target a specific PR number | - |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# Assign to a specific PR
review-dispatcher assign 4821

# Assign using global --pr flag
review-dispatcher --pr 4821 assign

# Assign with JSON output (for scripting)
review-dispatcher assign 4821 --json
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
