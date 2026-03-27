# files

**Show changed files for one or more PRs.**

Quickly see which files were modified before diving into the diff.

## When to Use

- Pre-review scan: "What did they change?"
- Impact assessment: "Which services are affected?"

## Synopsis

```bash
review-dispatcher files [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to show files for (shorthand) | - |
| `-p, --pr <PR_NUMBERS>` | PR number(s) to show files for (comma-separated) | - |
| `-a, --all` | Show files for all pending reviews | `false` |

## Examples

```bash
review-dispatcher files 4821
review-dispatcher files --pr 4821
review-dispatcher files --pr 4821,3156,2890
review-dispatcher files --all
```
