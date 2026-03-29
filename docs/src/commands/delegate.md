# delegate

**Ask Claude to triage each pending review — AI-powered PR analysis.**

Let Claude Code analyze your PRs, summarize changes, identify risks, and recommend actions.

## When to Use

- Morning triage: "Which PRs should I prioritize?"
- Before deep work: "Quick summary of what's waiting"
- Risk assessment: "Does this security-related PR need extra scrutiny?"

## Synopsis

```bash
prctrl delegate [OPTIONS]
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
| `--quiet`, `-q` | Suppress per-PR progress messages | `false` |
| `--pr-numbers` | PR number(s) to delegate (comma-separated) | - |
| `--repo <NAME>` | Filter by repository (partial match, case-insensitive) | - |
| `--author <NAME>` | Filter by author (partial match, case-insensitive) | - |

## Examples

```bash
# Interactive: select PR(s) to delegate
prctrl delegate

# Target a specific PR (global --pr flag)
prctrl delegate --pr 4821

# Preview what would be delegated (no actual delegation)
prctrl delegate --dry-run

# Preview specific PR without delegating
prctrl delegate --pr 4821 --dry-run

# Preview delegation with priority scores to identify urgent PRs first
prctrl delegate --dry-run --priority

# Only delegate recent PRs (last 7 days)
prctrl delegate --since-days 7

# Only delegate PRs from a specific repo
prctrl delegate --repo frontend

# Only delegate PRs from a specific author
prctrl delegate --author alice

# Combine filters for targeted delegation
prctrl delegate --repo api --author bob --dry-run

# JSON output for scripting
prctrl delegate --json

# Delegate all matching PRs without prompting (useful for scripts)
prctrl delegate --all

# Delegate all PRs from a specific repo without prompting
prctrl delegate --all --repo frontend

# Delegate specific PR and get JSON result
prctrl delegate --pr 4821 --json

# Quiet mode - suppresses per-PR output for cleaner batch processing
prctrl delegate --all --quiet

# Delegate multiple PRs at once with --pr-numbers
prctrl delegate --pr-numbers 4821,4822,4823 --all
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
