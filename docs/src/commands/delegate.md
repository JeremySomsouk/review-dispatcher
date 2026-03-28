# delegate

**Ask Claude to triage each pending review — AI-powered PR analysis.**

Let Claude Code analyze your PRs, summarize changes, identify risks, and recommend actions.

## When to Use

- Morning triage: "Which PRs should I prioritize?"
- Before deep work: "Quick summary of what's waiting"
- Risk assessment: "Does this security-related PR need extra scrutiny?"

## Synopsis

```bash
review-dispatcher delegate [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | Target specific PR (optional, shorthand for --pr) | All pending |
| `--json` | Output results as JSON (useful for scripting) | `false` |
| `--dry-run`, `-n` | Preview delegation without executing | `false` |
| `--priority`, `-P` | Show priority scores (1-5 stars) in dry-run output | `false` |

## Examples

```bash
# Interactive: select PR(s) to delegate
review-dispatcher delegate

# Target a specific PR
review-dispatcher delegate 4821

# Preview what would be delegated (no actual delegation)
review-dispatcher delegate --dry-run

# Preview specific PR without delegating
review-dispatcher delegate 4821 --dry-run

# Preview delegation with priority scores to identify urgent PRs first
review-dispatcher delegate --dry-run --priority

# JSON output for scripting
review-dispatcher delegate --json

# Delegate multiple PRs to Claude in parallel (JSON mode)
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
- In interactive mode, PRs are delegated sequentially with progress feedback
- In JSON mode, all PRs are delegated in parallel for speed
