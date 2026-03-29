# report

**Generate a weekly review report from processed review files.**

See your review activity over time — PRs reviewed, time spent, patterns.

## When to Use

- Weekly summary: "What did I review this week?"
- Team reporting: "Show my review output"
- Per-PR analysis: "Show me details for a specific PR"

## Synopsis

```bash
prctrl report [OPTIONS]
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
prctrl report
prctrl report --days 30
prctrl report --repo api
prctrl report --author alice --priority
prctrl report --days 14 --repo backend --priority
prctrl report --since-days 3 --priority
prctrl report --pr 123
prctrl report --pr 123 --json
prctrl report --pr-numbers 123,456,789
prctrl report --all --priority
prctrl report --all --repo myorg --since-days 7
```

## Tips

- Review files must exist in the output directory
- Use `clean` before generating a fresh report
- Use `--priority` to see which pending PRs are most urgent
- Use `--since-days` to filter pending PRs (consistent with `list`, `stats`, `delegate` commands)
- Use `[PR_NUMBER]` or `--pr` to target a specific PR (JSON output includes full PR details)
- Use `--pr-numbers` to target multiple specific PRs at once (fetches in parallel)
- Use `--all` to include all pending reviews in the report without prompting
