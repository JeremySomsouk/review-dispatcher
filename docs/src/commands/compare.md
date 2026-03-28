# compare

**Compare two PRs side-by-side to help decide which to review first.**

When you have multiple PRs competing for attention, `compare` gives you a head-to-head comparison based on age, size, complexity, and priority score.

## When to Use

- "Should I review #123 or #456 first?"
- Sprint planning: prioritize based on urgency and effort
- Code review load balancing: understand which PR is "cheaper" to review

## Synopsis

```bash
review-dispatcher compare <PR1> <PR2> [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `<PR1>` | First PR to compare (format: `repo#123` or just `123`) | Required |
| `<PR2>` | Second PR to compare (format: `repo#123` or just `123`) | Required |
| `-d, --detailed` | Show detailed comparison including language breakdown | `false` |
| `-P, --priority` | Show priority scores for each PR (1-5 stars based on age and size) | `false` |
| `--json` | Output as JSON for scripting | `false` |

## PR Format

PRs can be specified in two ways:

- **Full format**: `repo#123` (e.g., `frontend#4821`)
- **Short format**: `123` (uses first repo from config)

## Examples

```bash
# Compare two PRs in the same repo
review-dispatcher compare 123 456

# Compare PRs across different repos
review-dispatcher compare frontend#4821 backend#1024

# Detailed comparison with language breakdown
review-dispatcher compare 123 456 --detailed

# Show priority scores with stars
review-dispatcher compare 123 456 --priority

# JSON output for automation
review-dispatcher compare 123 456 --json
```

## Output

```
⚖️  PR Comparison
════════════════════════════════════════════════════════════

              #123                    #456
  ──────────────────────────────────────────────────────────
  Author       alice                  bob
  Age          5 days                 2 days
  Size         +340/-25               +1200/-200
  Files        12                     8
  Draft        No                     Yes
  Priority     4/5 ⭐⭐⭐⭐           3/5 ⭐⭐⭐

  ──────────────────────────────────────────────────────────
  📊 Summary:
    • Age: #456 wins → (newer)
    • Size: #123 wins → (smaller)
    • Priority: #123 wins → (higher score)

  💻 Languages:
    PR #123: TypeScript (8), JSON (3), Markdown (1)
    PR #456: Go (5), YAML (2), Shell (1)

  🔗 Links:
    PR #123: https://github.com/myorg/frontend/pull/123
    PR #456: https://github.com/myorg/backend/pull/456
```

## Comparison Factors

| Factor | Winner | Rationale |
|--------|--------|-----------|
| **Age** | Newer PR | Less time invested = less pressure |
| **Size** | Smaller PR | Fewer lines = faster to review |
| **Priority** | Higher score | Already calculated for urgency |

## Tips

- Use `review-dispatcher top` to find your highest-priority PRs first
- Combine with `browse` to open compared PRs directly
- For complex decisions, use `--detailed` to see language breakdown
- Use `--priority` to quickly see urgency scores as visual stars
- Consider draft PRs lower priority (they're still in progress)

## Related Commands

- [`top`](./top.md) — Show highest priority PRs
- [`focus`](./focus.md) — Show the single most urgent PR
- [`summary`](./summary.md) — Quick one-line overview
