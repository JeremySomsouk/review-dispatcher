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
| `PR_NUMBER` | PR number to show labels for (shorthand for --pr) | - |
| `-p, --pr <NUMBER>` | Target a specific PR by number (global) | - |
| `--pr-numbers <NUMS>` | PR number(s) to show labels for (comma-separated) | - |
| `-a, --all` | Show labels for all pending reviews | `false` |
| `-l, --filter-by <LABEL>` | Filter by label name (partial match, case-insensitive) | - |
| `--repo <REPO>` | Filter by repository name (partial match, case-insensitive) | - |
| `--author <USER>` | Filter by author username (partial match, case-insensitive) | - |
| `--json` | Output as JSON | `false` |
| `-P, --priority` | Show priority score for each PR (1-5 stars based on age and size) | `false` |

## Examples

```bash
# Show labels for a specific PR
review-dispatcher labels 4821

# Show labels for specific PR (using --pr flag)
review-dispatcher labels --pr 4821

# Show labels for multiple PRs
review-dispatcher labels --pr-numbers 4821,4822,4823

# Filter labels by name containing "security"
review-dispatcher labels -a -l security

# Filter by repository name
review-dispatcher labels --repo api-service

# Filter by author
review-dispatcher labels --author alice

# Combine filters
review-dispatcher labels -a --repo api --author alice

# Output as JSON for scripting
review-dispatcher labels --pr 4821 --json

# Show priority scores alongside labels
review-dispatcher labels -a --priority
