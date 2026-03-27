# ready

**Show PRs that are ready to merge — approved, CI passing, no conflicts.**

A merge-ready PR is one that has:
- ✅ Not a draft
- ✅ CI/CD checks passing (or pending)
- ✅ No merge conflicts
- ✅ Is mergeable (GitHub says so)

## When to Use

- Morning check: "Which PRs can I merge right now?"
- Release planning: "What's blocking the merge queue?"
- Dashboard prep: "Get a quick list of deployable changes"
- QA handoff: "Verify which PRs are good to go"

## Synopsis

```bash
review-dispatcher ready [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--repo <NAME>` | Filter to specific repository (partial match) | All repos |
| `--json` | Output as JSON for scripting | `false` |

## How It Works

The `ready` command analyzes your pending review PRs and checks each one's merge readiness:

1. **Fetches CI status** via GitHub's combined status API
2. **Checks merge conflicts** using GitHub's mergeable field
3. **Filters out drafts** (not ready for merge)
4. **Sorts by age** — oldest mergeable PRs first

A PR is considered "ready to merge" when:
- `draft = false`
- `ci_status = "success"` or `"pending"`
- `mergeable = true` (not `false`)
- `has_conflicts = false`

## Output

```
🚀 Merge Readiness — 8 PRs total, 3 ready to merge
──────────────────────────────────────────────────

  ✅  #4821  Fix authentication bug
      👤 alice  •  📦 +340/-25  •  ⏱️ 2 days  •  ✅ CI
      📁 myorg/frontend  🔗 https://github.com/myorg/frontend/pull/4821

  ✅  #4815  Update dependencies
      👤 bob  •  📦 +50/-10  •  ⏱️ today  •  ✅ CI
      📁 myorg/shared  🔗 https://github.com/myorg/shared/pull/4815

  ⏳  #4809  Refactor API gateway
      👤 carol  •  📦 +1200/-200  •  ⏱️ 5 days  •  ⏳ CI pending
      📁 myorg/backend  🔗 https://github.com/myorg/backend/pull/4809

──────────────────────────────────────────────────
  💡 Ready = not draft + CI passing + no conflicts
  💡 Use `--json` for scripting
```

## Examples

```bash
# Interactive: show all pending PRs with readiness status
review-dispatcher ready

# Filter to specific repo
review-dispatcher ready --repo frontend

# JSON output for scripting
review-dispatcher ready --json

# Combine with other commands
review-dispatcher ready --repo backend | grep "✅"
```

## Tips

- Use `--json` for integration with dashboards or automation scripts
- Pipe to `grep "✅"` to get just the ready PRs
- Ready PRs are sorted by age — oldest first
- CI "pending" is counted as ready (in progress, not failed)
- Combine with `browse` to quickly open merge-ready PRs

## Related Commands

- [`ci`](./ci.md) — Detailed CI/CD pipeline status
- [`conflicts`](./conflicts.md) — Find PRs with merge conflicts
- [`browse`](./browse.md) — Open PRs in browser
