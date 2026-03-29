# report

**Generate a weekly review report from processed review files.**

See your review activity over time — PRs reviewed, time spent, patterns.

## When to Use

- Weekly summary: "What did I review this week?"
- Team reporting: "Show my review output"

## Synopsis

```bash
review-dispatcher report [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-d, --days <NUM>` | Number of days to look back for processed reviews | `7` |
| `-s, --since-days <NUM>` | Only show pending PRs created since this many days ago | |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | |
| `-P, --priority` | Show priority breakdown for pending PRs | `false` |
| `--json` | Output as JSON | `false` |

## Examples

```bash
review-dispatcher report
review-dispatcher report --days 30
review-dispatcher report --repo api
review-dispatcher report --author alice --priority
review-dispatcher report --days 14 --repo backend --priority
review-dispatcher report --since-days 3 --priority
```

## Tips

- Review files must exist in the output directory
- Use `clean` before generating a fresh report
- Use `--priority` to see which pending PRs are most urgent
- Use `--since-days` to filter pending PRs (consistent with `list`, `stats`, `delegate` commands)
