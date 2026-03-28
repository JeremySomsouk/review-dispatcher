# info

**Show full PR information including description, reviewers, labels, and metadata.**

The `info` command displays comprehensive details about a PR that go beyond what `diff` shows — including the PR body/description, requested reviewers, teams, labels, assignees, and timestamps.

## When to Use

- Read the full PR description before starting a review
- Check who else is requested for review
- See PR labels and milestone context
- Get assignees information
- Verify PR metadata before commenting or approving

## Synopsis

```bash
review-dispatcher info [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | Target a specific PR by number | Interactive selection |
| `--pr, -p` | Target a specific PR by number (global) | Interactive selection |
| `--priority, -P` | Show priority score (1-5 stars based on age and size) | `false` |
| `--repo` | Filter by repository name (partial match, case-insensitive) | None |
| `--author` | Filter by author username (partial match, case-insensitive) | None |
| `--json` | Output as JSON for scripting | `false` |

## Examples

```bash
# Show info for a specific PR
review-dispatcher info --pr 4821

# Show info with priority score
review-dispatcher info --pr 4821 --priority

# Interactive mode (select from pending reviews)
review-dispatcher info

# Filter by repository
review-dispatcher info --repo frontend

# Filter by author
review-dispatcher info --author sarah_dev

# Filter by both repo and author
review-dispatcher info --repo myorg --author sarah_dev

# JSON output for integration
review-dispatcher info --pr 4821 --json
```

## Output

When run normally, `info` shows:

- **Header**: PR number and title
- **Author & Timestamps**: Who opened it, when, and last update time
- **Branch**: Source branch name
- **State**: DRAFT or OPEN status
- **Repository**: Full repo name
- **Priority Score** (with `--priority`): 1-5 stars based on age and size
- **Requested Reviewers**: Individual users requested
- **Requested Teams**: Team slugs requested
- **Labels**: All labels attached to the PR
- **Description**: Full PR body text (truncated at 50 lines)
- **URL**: Direct link to the PR

```
╔══════════════════════════════════════════════════════════╗
║ 📋 PR #4821 — feat: add CSV export                      ║
╠══════════════════════════════════════════════════════════╣
║                                                           ║
║   👤 Author:     sarah_dev                               ║
║   📅 Created:   2026-03-20 14:30 UTC                     ║
║   🔄 Updated:   2026-03-25 09:15 UTC                     ║
║   🌿 Branch:    feature/export                           ║
║   📊 State:     OPEN                                     ║
║   📁 Repository: myorg/frontend                          ║
║   ⭐ Priority:  4/5  ⭐⭐⭐⭐☆                            ║
║                                                           ║
║   👥 Requested Reviewers:                                ║
║     @reviewer1                                           ║
║     @reviewer2                                           ║
║                                                           ║
║   🏷️  Labels:                                            ║
║     • feature                                            ║
║     • exports                                            ║
║                                                           ║
║   📝 Description:                                         ║
║ ─────────────────────────────────────────────────────────║
║   This PR adds CSV export functionality to the           ║
║   dashboard. It includes:                                ║
║                                                           ║
║   - Export button in the UI                              ║
║   - Backend endpoint for CSV generation                  ║
║   - Tests for the export function                        ║
║ ─────────────────────────────────────────────────────────║
║                                                           ║
║   🔗 https://github.com/myorg/frontend/pull/4821         ║
╚══════════════════════════════════════════════════════════╝
```

## JSON Output

When `--json` is specified, outputs a structured JSON object:

```json
{
  "number": 4821,
  "title": "feat: add CSV export",
  "author": "sarah_dev",
  "body": "This PR adds CSV export...",
  "repo": "myorg/frontend",
  "url": "https://github.com/myorg/frontend/pull/4821",
  "branch": "feature/export",
  "state": "open",
  "created_at": "2026-03-20 14:30:00 UTC",
  "updated_at": "2026-03-25 09:15:00 UTC",
  "additions": 245,
  "deletions": 32,
  "requested_reviewers": ["reviewer1", "reviewer2"],
  "requested_teams": ["team-slug"],
  "labels": ["feature", "exports"],
  "assignees": [],
  "priority_score": 4
}
```

## Comparison with `diff`

| Aspect | `diff` | `info` |
|--------|--------|--------|
| PR body/description | ❌ | ✅ |
| Requested reviewers | ❌ | ✅ |
| Labels | ❌ | ✅ |
| Assignees | ❌ | ✅ |
| Timestamps (created/updated) | Partial | ✅ Full |
| Diff/stats summary | ✅ | ✅ |

## Tips

- Use `--json` for integration with scripts or other tools
- The description is truncated at 50 lines for terminal display, but full body is shown in JSON
- Pair with `claim` to assign yourself and then `info` to read the full context
