# report

**Generate a weekly review report from processed review files.**

See your review activity over time — PRs reviewed, time spent, patterns.

## When to Use

- Weekly summary: "What did I review this week?"
- Team reporting: "Show my review output"
- Per-PR analysis: "Show me details for a specific PR"

## Synopsis

```bash
review-dispatcher report [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `[PR_NUMBER]` | Target a specific PR by number (shorthand for `--pr`) | |
| `--pr-numbers <NUMS>` | Target specific PRs by number (comma-separated) | |
| `-a, --all` | Include all pending reviews in the report | `false` |
| `-d, --days <NUM>` | Number of days to look back for processed reviews | `7` |
| `-s, --since-days <NUM>` | Only show pending PRs created since this many days ago | |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | |
| `-P, --priority` | Show priority breakdown for pending PRs | `false` |
| `--json` | Output as JSON (includes filtered PR details when using `--pr`) | `false` |

## Examples

```bash
review-dispatcher report
review-dispatcher report --days 30
review-dispatcher report --repo api
review-dispatcher report --author alice --priority
review-dispatcher report --days 14 --repo backend --priority
review-dispatcher report --since-days 3 --priority
review-dispatcher report --pr 123
review-dispatcher report --pr 123 --json
review-dispatcher report --pr-numbers 123,456,789
review-dispatcher report --all --priority
review-dispatcher report --all --repo myorg --since-days 7
```

## Tips

- Review files must exist in the output directory
- Use `clean` before generating a fresh report
- Use `--priority` to see which pending PRs are most urgent
- Use `--since-days` to filter pending PRs (consistent with `list`, `stats`, `delegate` commands)
- Use `[PR_NUMBER]` or `--pr` to target a specific PR (JSON output includes full PR details)
- Use `--pr-numbers` to target multiple specific PRs at once (fetches in parallel)
- Use `--all` to include all pending reviews in the report without prompting
