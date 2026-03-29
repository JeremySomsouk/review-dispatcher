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
prctrl export [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-f, --format <FORMAT>` | Output format: `csv`, `markdown`, or `json` | `csv` |
| `-o, --output <PATH>` | Write to file instead of stdout | stdout |
| `-c, --columns <COLS>` | Columns to include (comma-separated) | all |
| `PR_NUMBER` | Export specific PR by number (shorthand for --pr) | none |
| `-p, --pr <PR>` | Export specific PR (shorthand for --pr) | none |
| `--pr-numbers <NUMS>` | Export specific PRs (comma-separated PR numbers) | none |
| `-a, --all` | Fetch fresh data for all reviews | current session |
| `--json` | Output as JSON (useful for scripting) | false |
| `-P, --priority` | Show priority scores (1-5 stars based on age and size) | false |
| `--repo <REPO>` | Filter by repository name (partial match, case-insensitive) | none |
| `--author <USER>` | Filter by author username (partial match, case-insensitive) | none |
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | none |

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
| `priority` | Priority score (1-5) |

## Examples

```bash
# Export all pending reviews as CSV (prints to stdout)
prctrl export

# Export to a file
prctrl export --output pending-reviews.csv

# Export as Markdown table (great for Notion/Slack)
prctrl export --format markdown --output report.md

# Export only specific columns
prctrl export --columns repo,title,author,age

# Export with priority scores included
prctrl export --priority --output prioritized-reviews.csv

# Export as JSON for scripting or API integration
prctrl export --json --output reviews.json

# Export as JSON with priority scores
prctrl export --json --priority --output reviews.json

# Export with fresh data (bypasses session cache)
prctrl export --all --output full-report.csv

# Export only PRs from a specific repository
prctrl export --repo backend --output backend-reviews.csv

# Export only PRs by a specific author
prctrl export --author alice --output alice-reviews.csv

# Export only PRs created in the last 7 days
prctrl export --since-days 7 --output recent-reviews.csv

# Export a specific PR by number
prctrl export --pr 4821

# Export multiple specific PRs
prctrl export --pr-numbers 4821,3156,2890

# Combine filters with other options
prctrl export --repo frontend --author alice --format markdown --output report.md
```

## Output Examples

### CSV Format (default)

```csv
repo,number,title,author,size,age,draft,url
backend,4821,feat: add dark mode,alice,+89/-12,2d,no,https://github.com/org/backend/pull/4821
frontend,3156,fix: login timeout,bob,+234/-45,4d,no,https://github.com/org/frontend/pull/3156
```

### CSV with Priority

```csv
repo,number,title,author,size,age,draft,url,priority
backend,4821,feat: add dark mode,alice,+89/-12,2d,no,https://github.com/org/backend/pull/4821,3
frontend,3156,fix: login timeout,bob,+234/-45,4d,no,https://github.com/org/frontend/pull/3156,4
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
    "url": "https://github.com/org/backend/pull/4821",
    "priority_score": 3
  }
]
```

## Tips

- Use `--output` to write directly to a file instead of piping
- Markdown format is great for pasting into Slack messages or Notion docs
- JSON format is ideal for scripting and API integrations
- Combine with `--priority` to include urgency scores in exports
- Combine with `--all` to ensure you have the latest data
- Column order is preserved in CSV/Markdown output
