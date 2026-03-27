# blocked

**Show PRs that are blocked from merging — CI failures, merge conflicts, or other issues.**

Find PRs that can't be merged yet and see exactly what's blocking them.

## When to Use

- Merge queue planning: "What's blocking our PRs?"
- CI debugging: "Why is this PR blocked?"
- Conflicts check: "Do we have merge conflicts to resolve?"

## Synopsis

```bash
review-dispatcher blocked [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-r, --repo <NAME>` | Filter to specific repository | All repos |
| `-c, --ci-only` | Only show PRs with failing CI | `false` |
| `-m, --conflicts-only` | Only show PRs with merge conflicts | `false` |
| `-n, --limit <NUM>` | Maximum results shown | `20` |
| `--json` | Output as JSON | `false` |

## Examples

```bash
# Show all blocked PRs
review-dispatcher blocked

# Show only PRs with CI failures
review-dispatcher blocked --ci-only

# Show only PRs with merge conflicts
review-dispatcher blocked --conflicts-only

# Filter to specific repo
review-dispatcher blocked --repo frontend

# JSON output for scripting
review-dispatcher blocked --json
```
