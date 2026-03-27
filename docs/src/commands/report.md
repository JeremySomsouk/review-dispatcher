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
| `-d, --days <NUM>` | Number of days to look back | `7` |
| `--json` | Output as JSON | `false` |

## Examples

```bash
review-dispatcher report
review-dispatcher report --days 30
```

## Tips

- Review files must exist in the output directory
- Use `clean` before generating a fresh report
