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
| `PR_NUMBER` | Target specific PR (optional) | All pending |

## Examples

```bash
review-dispatcher delegate
review-dispatcher delegate 4821
```

## Tips

- Create `instruction.md` for project-specific review criteria
