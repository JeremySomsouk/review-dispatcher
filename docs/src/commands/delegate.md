# delegate

**Ask Claude to triage each pending review — AI-powered PR analysis.**

Let Claude Code analyze your PRs, summarize changes, identify risks, and recommend actions.

## When to Use

- Morning triage: "Which PRs should I prioritize?"
- Before deep work: "Quick summary of what's waiting"
- Risk assessment: "Does this security-related PR need extra scrutiny?"

## Synopsis

```bash
review-dispatcher delegate [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--pr`, `-p` | Target specific PR by number (global flag) | All pending |
| `--all`, `-a` | Delegate all matching PRs without prompting | `false` |
| `--json` | Output results as JSON (useful for scripting) | `false` |
| `--dry-run`, `-n` | Preview delegation without executing | `false` |
| `--priority`, `-P` | Show priority scores (1-5 stars) in dry-run output | `false` |
| `--since-days`, `-s` | Only delegate PRs created since N days ago | All |
| `--repo <NAME>` | Filter by repository (partial match, case-insensitive) | - |
| `--author <NAME>` | Filter by author (partial match, case-insensitive) | - |

## Examples

```bash
# Interactive: select PR(s) to delegate
review-dispatcher delegate

# Target a specific PR (global --pr flag)
review-dispatcher delegate --pr 4821

# Preview what would be delegated (no actual delegation)
review-dispatcher delegate --dry-run

# Preview specific PR without delegating
review-dispatcher delegate --pr 4821 --dry-run

# Preview delegation with priority scores to identify urgent PRs first
review-dispatcher delegate --dry-run --priority

# Only delegate recent PRs (last 7 days)
review-dispatcher delegate --since-days 7

# Only delegate PRs from a specific repo
review-dispatcher delegate --repo frontend

# Only delegate PRs from a specific author
review-dispatcher delegate --author alice

# Combine filters for targeted delegation
review-dispatcher delegate --repo api --author bob --dry-run

# JSON output for scripting
review-dispatcher delegate --json

# Delegate all matching PRs without prompting (useful for scripts)
review-dispatcher delegate --all

# Delegate all PRs from a specific repo without prompting
review-dispatcher delegate --all --repo frontend

# Delegate specific PR and get JSON result
review-dispatcher delegate --pr 4821 --json
```

## Output

### Interactive Mode (default)
Each delegation shows:
- Delegation progress indicator
- Summary from Claude
- Review file saved to output directory

### JSON Mode
Returns an array of results with:
```json
[
  {
    "pr_number": 4821,
    "pr_title": "feat: add dark mode",
    "repo": "frontend",
    "url": "https://github.com/org/frontend/pull/4821",
    "success": true,
    "summary": "Summary text from Claude",
    "error": null
  }
]
```

## Tips

- Create `instruction.md` for project-specific review criteria
- Use `--dry-run` to verify targeting before committing to delegation
- Use `--json` for automation scripts or piping to other tools
- All PRs are delegated in parallel for speed (both modes)
- Progress feedback shows completion status for each PR as it finishes
- Combine filters (`--since-days`, `--repo`, `--author`) with `--dry-run` to preview targeted delegation
- Snoozed PRs are automatically excluded (consistent with `list` command)
- When `--pr` is combined with `--repo`, `--author`, or `--since-days`, the PR must match all filters to be selected (filters take precedence over direct fetch)
