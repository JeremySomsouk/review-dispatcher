# browse

**Open one or more PRs in your browser.**

Jump straight to GitHub without switching windows or copying URLs.

## When to Use

- After triage: "Let me see the actual code"
- Quick access: "Open all my pending PRs"

## Synopsis

```bash
review-dispatcher browse [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to open (shorthand) | - |
| `-p, --pr <PR_NUMBERS>` | PR number(s) to open (comma-separated) | - |

## Examples

```bash
review-dispatcher browse 4821
review-dispatcher browse --pr 4821
review-dispatcher browse --pr 4821,3156,2890
```
