# labels

**Show labels for one or more PRs.**

Quickly see what tags and categories are applied to your pending PRs.

## When to Use

- Filtering: "Find all 'security' PRs"
- Organization: "What labels are being used?"

## Synopsis

```bash
prctrl labels [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to show labels for (shorthand for --pr) | - |
| `-p, --pr <NUMBER>` | Target a specific PR by number (global) | - |
| `--pr-numbers <NUMS>` | PR number(s) to show labels for (comma-separated) | - |
| `-a, --all` | Show labels for all pending reviews | `false` |
| `-l, --filter-by <LABEL>` | Filter by label name (partial match, case-insensitive) | - |
| `--repo <REPO>` | Filter by repository name (partial match, case-insensitive) | - |
| `--author <USER>` | Filter by author username (partial match, case-insensitive) | - |
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | - |
| `--json` | Output as JSON | `false` |
| `-P, --priority` | Show priority score for each PR (1-5 stars based on age and size) | `false` |

## Examples

```bash
# Show labels for a specific PR
prctrl labels 4821

# Show labels for specific PR (using --pr flag)
prctrl labels --pr 4821

# Show labels for multiple PRs
prctrl labels --pr-numbers 4821,4822,4823

# Filter labels by name containing "security"
prctrl labels -a -l security

# Filter by repository name
prctrl labels --repo api-service

# Filter by author
prctrl labels --author alice

# Show labels for all PRs from the last 7 days
prctrl labels -a --since-days 7

# Combine filters
prctrl labels -a --repo api --author alice --since-days 14

# Output as JSON for scripting
prctrl labels --pr 4821 --json

# Show priority scores alongside labels
prctrl labels -a --priority
