# review

**Fetch and display a PR diff in the terminal with syntax highlighting.**

Full code review in your terminal — no browser needed.

## When to Use

- Deep dive: "I need to see the actual code"
- Offline review: "No browser, but need to review"

## Synopsis

```bash
review-dispatcher review [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to review | Required (or use `--pr` or `--all`) |
| `-n, --pr-numbers <NUMS>` | PR number(s) to review (comma-separated) | - |
| `-p, --pr <NUM>` | Target specific PR by number | - |
| `-a, --all` | Show diffs for all pending reviews | `false` |
| `-n, --dry-run` | Preview which PRs would be shown without displaying diffs | `false` |
| `-c, --context <NUM>` | Context lines around changes | `3` |
| `-o, --output <FILE>` | Output diff to file | Terminal |
| `-l, --language <LANG>` | Language hint for syntax highlighting | Auto-detected |
| `-P, --priority` | Show priority score (1-5 stars) | `false` |
| `--repo <PATTERN>` | Filter by repository (partial match, case-insensitive) | - |
| `--author <PATTERN>` | Filter by author (partial match, case-insensitive) | - |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
review-dispatcher review 4821
review-dispatcher review 4821 --context 5
review-dispatcher review --priority 4821
review-dispatcher review 4821 --json
review-dispatcher review --pr 4821 --output diff.patch
review-dispatcher review --all
review-dispatcher review --all --priority
review-dispatcher review --all --repo myservice
review-dispatcher review --all --author johndoe
review-dispatcher review --all --repo api --author alice
review-dispatcher review --pr-numbers 4821,4822,4823
review-dispatcher review --pr-numbers 4821,4822 --priority
review-dispatcher review --all --dry-run
review-dispatcher review --pr 4821 --dry-run
```
