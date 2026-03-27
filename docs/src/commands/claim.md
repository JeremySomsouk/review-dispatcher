# claim

**Claim multiple PRs for review at once.**

Sign up for review responsibility without clicking through the web UI.

## When to Use

- Sprint start: "I'll take these three"
- Batch workflow: "Claim, review, repeat"

## Synopsis

```bash
review-dispatcher claim [OPTIONS] [PR_NUMBERS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-a, --all` | Claim all pending reviews | `false` |
| `--json` | Output results as JSON | `false` |
| `PR_NUMBERS` | PR number(s) to claim (comma-separated) | - |

## Examples

```bash
review-dispatcher claim 4821,3156,2890
review-dispatcher claim --all
```
