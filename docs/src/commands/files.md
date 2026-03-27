# files

**Show changed files for one or more PRs.**

Quickly see which files were modified before diving into the diff.

## When to Use

- Pre-review scan: "What did they change?"
- Impact assessment: "Which services are affected?"

## Synopsis

```bash
review-dispatcher files [OPTIONS] [PR_NUMBERS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBERS` | PR number(s) to show files for | - |
| `-a, --all` | Show files for all pending reviews | `false` |

## Examples

```bash
review-dispatcher files 4821
review-dispatcher files --all
```
