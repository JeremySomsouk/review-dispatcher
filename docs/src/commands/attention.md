# attention

**Analyze which PRs demand your immediate attention based on multiple urgency factors.**

When you're overwhelmed by pending reviews, `attention` cuts through the noise by calculating a multi-factor attention score for each PR. Unlike the simple priority score, it combines age, size, staleness, and draft status into a comprehensive urgency metric (1-10 scale).

## When to Use

- Morning triage: "Which PRs should I prioritize today?"
- End-of-day review: "What did I miss that needs urgent attention?"
- Sprint planning: "Which PRs are becoming critical?"
- When `focus` is too narrow and you need a broader view of urgency

## Synopsis

```bash
review-dispatcher attention [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-t, --threshold <NUM>` | Minimum attention score to show (1-10) | `5` |
| `-d, --detailed` | Show detailed score breakdown | `false` |
| `-n, --limit <NUM>` | Limit the number of results shown | `10` |
| `-P, --priority` | Show priority score (1-5 stars based on age and size) | `false` |
| `--repo <PATTERN>` | Filter by repository name (partial match, case-insensitive) | |
| `--author <PATTERN>` | Filter by author username (partial match, case-insensitive) | |
| `--json` | Output as JSON for scripting | `false` |

## Attention Score Calculation

The attention score (1-10) combines four factors:

| Factor | Range | Description |
|--------|-------|-------------|
| Age | 1-5 | How long the PR has been waiting (newer=lower, older=higher) |
| Size | 1-3 | Total lines changed (+additions -deletions) |
| Draft status | 1-2 | Draft PRs score lower (less urgent) |
| Staleness bonus | 0-2 | Extra urgency for PRs waiting >7 days |

### Score Thresholds

| Score Range | Urgency | Interpretation |
|-------------|---------|----------------|
| 8-10 | 🔥 Critical | Needs immediate attention |
| 6-7 | ⚡ High | Should review soon |
| 4-5 | 📅 Medium | Normal priority |
| 1-3 | 💤 Low | Can wait |

## Examples

```bash
# See your most attention-demanding PRs
review-dispatcher attention

# Show only critical PRs (score >= 8)
review-dispatcher attention --threshold 8

# Show detailed breakdown of why each PR scored high
review-dispatcher attention --detailed

# Show top 5 most urgent
review-dispatcher attention --limit 5

# Show with priority scores (1-5 stars)
review-dispatcher attention --priority

# Filter by repository
review-dispatcher attention --repo myorg/frontend

# Filter by author
review-dispatcher attention --author alice

# Combine filters
review-dispatcher attention --repo myorg --author alice --priority

# Get JSON for scripting or dashboards
review-dispatcher attention --json
```

## Output

```
🎯 3 PR(s) demand your attention (score >= 5)

  🔥🔥🔥  Fix authentication bug  #4821  (myorg/frontend)
      👤 alice  •  340 lines  •  opened 5d
      🔗 https://github.com/myorg/frontend/pull/4821

  🔥🔥  Refactor API gateway  #4815  (myorg/backend)
      👤 bob  •  890 lines  •  opened 3d
      🔗 https://github.com/myorg/backend/pull/4815

  🔥🔥  Update dependencies  #4809  (myorg/shared)
      👤 carol  •  120 lines  •  opened 2d
      🔗 https://github.com/myorg/shared/pull/4809
```

### With `--detailed` flag

```
  🔥🔥🔥  Fix authentication bug  #4821  (myorg/frontend)
      👤 alice  •  340 lines  •  opened 5d
      📊 breakdown: age=3 size=2 draft=2 stale_bonus=1
      🔗 https://github.com/myorg/frontend/pull/4821
```

## Tips

- Use `--threshold 8` to see only truly critical PRs
- Combine with `--detailed` when explaining to others why a PR is urgent
- Use `--json` for integration with external tools or Slack notifications
- Pair with `focus --open` to immediately start reviewing the most urgent PR

## Related Commands

- [`focus`](./focus.md) — Show the single most urgent PR
- [`summary`](./summary.md) — Quick one-line overview
- [`top`](./top.md) — Highest priority PRs by score
