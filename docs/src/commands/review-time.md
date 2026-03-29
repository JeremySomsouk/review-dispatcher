# review-time

**Estimate how long each PR will take to review based on size and complexity.**

Review time estimation helps you plan your review sessions and prioritize efficiently.

## When to Use

- Morning planning: "What can I realistically review today?"
- Time-boxing: "I have 2 hours before lunch — which PRs fit?"
- Sprint planning: "Budget review time across the sprint"
- Delegation: "Which PRs can I handle vs. should delegate?"

## Synopsis

```bash
prctrl review-time [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to estimate review time for (shorthand for --pr) | Interactive |
| `-p, --pr <PR>` | Show review time for specific PR (shorthand for --pr) | Interactive |
| `PR_NUMBERS` | PR number(s) to estimate (comma-separated) | Interactive |
| `-a, --all` | Show estimates for all pending reviews | `false` |
| `-g, --grouped` | Group output by time category | `false` |
| `-P, --priority` | Show priority scores for each PR | `false` |
| `--repo <REPO>` | Filter by repository name (partial match, case-insensitive) | None |
| `--author <AUTHOR>` | Filter by author username (partial match, case-insensitive) | None |
| `-s, --since-days <DAYS>` | Only show PRs created since this many days ago | None |
| `--json` | Output as JSON for scripting | `false` |

## How It Works

The estimation algorithm considers:

1. **Lines changed** — Base formula: ~2 min per 50 lines
2. **File count** — Number of files changed (fetched in parallel for performance)
3. **Size complexity** — Larger PRs get a complexity multiplier:
   - XS (<50 lines): 0.8x (quick win)
   - S (50-200 lines): 1.0x (standard)
   - M (200-500 lines): 1.2x (moderate complexity)
   - L (500-1000 lines): 1.5x (higher complexity)
   - XL (1000+ lines): 2.0x (large, likely complex)
4. **Age factor** — PRs older than 14 days get 0.9x (context lost)
5. **Minimum floor** — No estimate under 5 minutes

## Time Categories

| Category | Time Range | Indicator |
|----------|------------|-----------|
| ⚡ lightning | < 10 min | Quick review |
| 🔵 quick | 10-20 min | Standard small PR |
| 🟡 moderate | 20-45 min | Medium complexity |
| 🟠 substantial | 45-90 min | Large or complex |
| 🔴 lengthy | > 90 min | Major undertaking |

## Examples

```bash
# Interactive: select PRs from pending list
prctrl review-time

# Target specific PR directly (consistent with other commands)
prctrl review-time --pr 4821
prctrl review-time -p 4821

# Estimate for specific PRs via positional arg
prctrl review-time 4821
prctrl review-time 4821,4815,4809

# Estimate all pending reviews
prctrl review-time --all

# Group by time category for session planning
prctrl review-time --all --grouped

# Show priority scores with estimates
prctrl review-time --all --priority

# Filter by repository
prctrl review-time --all --repo my-repo

# Filter by author
prctrl review-time --all --author alice

# Combine filters
prctrl review-time --all --repo api --author alice --priority

# Filter by age
prctrl review-time --all --since-days 7

# Get JSON output for scripting
prctrl review-time --all --json
```

## Output

```
⏱️  Review Time Estimates — 5 PRs, ~2.5h total
───────────────────────────────────────
  M  Fix authentication bug  #4821  myorg/repo
     👤 alice  •  📦 M (340 lines)  •  ⏱️ 15 min  🟡 moderate
     ★★★☆☆  ⭐3

  XL Refactor API gateway  #4815  myorg/repo
     👤 bob  •  📦 XL (1200 lines)  •  ⏱️ 1.8h  🔴 lengthy
     ★★★★★  ⭐5
...
```

## Tips

- Sort by time descending (longest first) for session planning
- Combine with `--json` for spreadsheet导入
- Use `quick` category PRs forbetween-meeting slots
- Large PRs (>1h) are good candidates for delegation or pairing
