# export

**Export pending reviews to CSV, Markdown, or JSON format.**

Perfect for manager reports, team dashboards, or integrating with other tools.

## When to Use

- Weekly reports: "Export my queue for the team"
- Spreadsheet analysis: "Open in Excel/Google Sheets"
- Team dashboards: "Post to Slack/Notion"
- Manager summaries: "Show backlog size and aging"
- Scripting: "Parse exports in automated workflows"

## Synopsis

```bash
review-dispatcher export [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-f, --format <FORMAT>` | Output format: `csv` or `markdown` | `csv` |
| `-o, --output <PATH>` | Write to file instead of stdout | stdout |
| `-c, --columns <COLS>` | Columns to include (comma-separated) | all |
| `-a, --all` | Fetch fresh data for all reviews | current session |
| `--json` | Output as JSON (useful for scripting) | false |

### Available Columns

| Column | Description |
|--------|-------------|
| `repo` | Repository name |
| `number` | PR number |
| `title` | PR title |
| `author` | PR author username |
| `size` | Lines changed (+additions/-deletions) |
| `age` | Age in days |
| `draft` | Whether PR is a draft |
| `url` | Full GitHub URL to PR |

## Examples

```bash
# Export all pending reviews as CSV (prints to stdout)
review-dispatcher export

# Export to a file
review-dispatcher export --output pending-reviews.csv

# Export as Markdown table (great for Notion/Slack)
review-dispatcher export --format markdown --output report.md

# Export only specific columns
review-dispatcher export --columns repo,title,author,age

# Export with fresh data (bypasses session cache)
review-dispatcher export --all --output full-report.csv

# Export as JSON for scripting or API integration
review-dispatcher export --json --output reviews.json
```

## Output Examples

### CSV Format (default)

```csv
repo,number,title,author,size,age,draft,url
backend,4821,feat: add dark mode,alice,+89/-12,2d,no,https://github.com/org/backend/pull/4821
frontend,3156,fix: login timeout,bob,+234/-45,4d,no,https://github.com/org/frontend/pull/3156
```

### Markdown Format

```markdown
| Repo | # | Title | Author | Size | Age | Draft | URL |
|------|---|-------|--------|------|-----|-------|-----|
| `backend` | #4821 | feat: add dark mode | alice | +89/-12 | 2d | no | [link](https://...) |
```

### JSON Format

```json
[
  {
    "repo": "backend",
    "number": 4821,
    "title": "feat: add dark mode",
    "author": "alice",
    "size": "+89/-12",
    "age_days": 2,
    "draft": false,
    "url": "https://github.com/org/backend/pull/4821"
  }
]
```

## Tips

- Use `--output` to write directly to a file instead of piping
- Markdown format is great for pasting into Slack messages or Notion docs
- JSON format is ideal for scripting and API integrations
- Combine with `--all` to ensure you have the latest data
- Column order is preserved in CSV/Markdown output
