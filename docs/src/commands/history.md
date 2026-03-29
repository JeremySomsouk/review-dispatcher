# history

**Search and filter your review history from processed review files.**

The `history` command lets you dig into your past review activity with powerful filtering — by repository, author, review state (approved/changes requested/commented), and time window. It reads from the review files created when you run `prctrl list` or delegate reviews.

## When to Use

- Research: "Which PRs did I review for the frontend team last month?"
- Author follow-up: "What reviews did I give to alice's PRs this week?"
- State analysis: "Show me all the PRs where I requested changes"
- Reporting: "Generate a list of all my reviews for the past quarter"

## Synopsis

```bash
prctrl history [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--repo <NAME>` | Filter by repository name (partial match, case-insensitive) | All repos |
| `--author <NAME>` | Filter by PR author username (partial match, case-insensitive) | All authors |
| `-s, --state <STATE>` | Filter by review state: `APPROVED`, `CHANGES_REQUESTED`, `COMMENTED` | All states |
| `-d, --days <NUM>` | Number of days to look back | `30` |
| `-n, --limit <NUM>` | Limit the number of results shown | `50` |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# View your last 30 days of review history
prctrl history

# Show only reviews for the frontend repo
prctrl history --repo frontend

# Show reviews where you requested changes
prctrl history --state CHANGES_REQUESTED

# Find all reviews of alice's PRs in the last 90 days
prctrl history --author alice --days 90

# Combine filters - reviews of bob's PRs in backend repo with changes requested
prctrl history --author bob --repo backend --state CHANGES_REQUESTED

# Get JSON for scripting
prctrl history --json --days 7

# Limit to 10 most recent
prctrl history --limit 10
```

## Output

```
📜 Review History (last 30 days)
──────────────────────────────────────────────────
  Total matching entries: 47

  ✅ APPROVED (32 PRs)
    #4821  Fix authentication bug           myorg/frontend (2d ago)
    #4815  Update dependencies              myorg/shared (3d ago)
    ...

  🔁 CHANGES_REQUESTED (10 PRs)
    #4802  Refactor API gateway            myorg/backend (5d ago)
    ...

  💬 COMMENTED (5 PRs)
    #4798  Add new feature                  myorg/frontend (1w ago)
    ...

──────────────────────────────────────────────────
  💡 Use `--json` for scripting | `--repo`, `--author`, `--state` to filter
```

## Tips

- Review history is built from `.md` files in your output directory (default: `./reviews`)
- Run `prctrl list` first to populate review files
- Filter combinations are ANDed together (repo AND author AND state)
- Use `--json` for integration with external tools or dashboards
- The `--limit` flag helps when you just need the most recent N entries

## Related Commands

- [`activity`](./activity.md) — GitHub-sourced review activity (uses GitHub API directly)
- [`report`](./report.md) — Weekly review summary reports
- [`stats`](./stats.md) — Statistics about pending reviews
