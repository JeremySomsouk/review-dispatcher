# labels

**Show labels for one or more PRs.**

Quickly see what tags and categories are applied to your pending PRs.

## When to Use

- Filtering: "Find all 'security' PRs"
- Organization: "What labels are being used?"

## Synopsis

```bash
review-dispatcher labels [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to show labels for | - |
| `-n, --pr-numbers <NUMS>` | PR number(s) to show labels for (comma-separated) | - |
| `-a, --all` | Show labels for all pending reviews | `false` |
| `-l, --filter-by <LABEL>` | Filter by label name (partial match, case-insensitive) | - |
| `--json` | Output as JSON | `false` |

## Examples

```bash
# Show labels for a specific PR
review-dispatcher labels 4821

# Show labels for multiple PRs
review-dispatcher labels -p 4821,4822,4823

# Filter labels by name containing "security"
review-dispatcher labels -a -l security

# Output as JSON for scripting
review-dispatcher labels -p 4821,4822 --json
```
