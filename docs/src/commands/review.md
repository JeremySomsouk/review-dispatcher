# review

**Fetch and display a PR diff in the terminal with syntax highlighting.**

Full code review in your terminal — no browser needed.

## When to Use

- Deep dive: "I need to see the actual code"
- Offline review: "No browser, but need to review"

## Synopsis

```bash
prctrl review [OPTIONS] [PR_NUMBER]
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
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | - |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
prctrl review 4821
prctrl review 4821 --context 5
prctrl review --priority 4821
prctrl review 4821 --json
prctrl review --pr 4821 --output diff.patch
prctrl review --all
prctrl review --all --priority
prctrl review --all --repo myservice
prctrl review --all --author johndoe
prctrl review --all --repo api --author alice
prctrl review --pr-numbers 4821,4822,4823
prctrl review --pr-numbers 4821,4822 --priority
prctrl review --all --dry-run
prctrl review --pr 4821 --dry-run
prctrl review --all --since-days 7
prctrl review --all --repo api --since-days 14
```
