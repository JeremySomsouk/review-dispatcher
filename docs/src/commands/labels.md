# labels

**Show labels for one or more PRs.**

Quickly see what tags and categories are applied to your pending PRs.

## When to Use

- Filtering: "Find all 'security' PRs"
- Organization: "What labels are being used?"

## Synopsis

```bash
review-dispatcher labels [OPTIONS] [PR_NUMBERS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBERS` | PR number(s) to show labels for | - |
| `-a, --all` | Show labels for all pending reviews | `false` |
| `-f, --filter <LABEL>` | Filter by label name | - |

## Examples

```bash
review-dispatcher labels 4821
review-dispatcher labels --filter security
```
