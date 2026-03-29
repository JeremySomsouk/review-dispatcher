# chase

**Send follow-up reminders to authors of stale PRs to get their attention.**

Nothing kills momentum faster than PRs that languish without review. `chase` helps you send gentle, polite reminders to authors whose PRs have been waiting too long — either as a preview (default) or actually sent to GitHub.

## When to Use

- Weekly maintenance: "Time to shake the trees a bit"
- Sprint end: "Let's close out those pending reviews"
- Post-vacation: "Get eyes back on neglected PRs"
- Custom follow-up: With your own message template

## Synopsis

```bash
review-dispatcher chase [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to chase (shorthand for `--pr`) | - |
| `-p, --pr <PR>` | Target a specific PR by number | - |
| `-a, --min-age <DAYS>` | Minimum age in days to chase (default: 7) | `7` |
| `-s, --since-days <DAYS>` | Only chase PRs created since this many days ago | - |
| `--send` | Actually post comments to GitHub (default: preview only) | `false` |
| `-m, --message <TEXT>` | Custom message template | Default template |
| `--repo <REPO>` | Filter by repository name (partial match, case-insensitive) | - |
| `--author <USER>` | Filter by author username (partial match, case-insensitive) | - |
| `-P, --priority` | Show priority scores for each PR (1-5 stars based on age and size) | `false` |
| `--json` | Output as JSON | `false` |

## Message Template

The default message is:

```
👋 Hi @{author}! Just checking in on this PR — it's been waiting for 
review for {days} days. Could you please address any pending feedback 
or let us know if it's ready for another look? Thanks!
```

### Template Variables

Your custom message can include these placeholders:

| Variable | Description |
|----------|-------------|
| `{author}` | PR author's username |
| `{title}` | PR title |
| `{days}` | Days waiting for review |
| `{repo}` | Repository name |
| `{pr}` | PR number with # prefix |

## Examples

```bash
# Preview chase comments for PRs older than 7 days
review-dispatcher chase

# Chase a specific PR (ignores --min-age, targets only that PR)
review-dispatcher chase --pr 123

# Chase PRs older than 14 days and post comments
review-dispatcher chase --min-age 14 --send

# Chase only PRs created in the last 3 days (newer stale PRs)
review-dispatcher chase --since-days 3

# Chase PRs between 3-14 days old
review-dispatcher chase --since-days 14 --min-age 3

# Chase with priority scores to identify most urgent PRs
review-dispatcher chase --priority

# Use a custom message template
review-dispatcher chase --message "Hey {author}, bumping this - it's been {days} days!"

# Get JSON output for scripting
review-dispatcher chase --json
```

## Output

When run in preview mode (without `--send`):

- Shows each stale PR with author, age, and proposed comment
- Color-coded age badges (red for 14+ days, yellow for 7+ days)
- Priority scores (when `--priority` is specified)
- Summary of how many PRs would be chased

When run with `--send`:

- Posts the comment to each PR on GitHub
- Shows success/failure for each
- Final count of sent vs. failed

## Safety

**Dry-run by default**: The `--send` flag is required to actually post comments. This prevents accidental spam.

## Tips

- Start with a dry run to review the message tone
- Use `--min-age 14` for weekly maintenance routines
- Custom messages are great for team-specific workflows
- Combine with `catchup` for a comprehensive review maintenance session
