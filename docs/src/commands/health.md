# health

**Show GitHub API health status and rate limits.**

Avoid hitting rate limits by checking before large operations.

## When to Use

- Before batch operations: "Will I hit the limit?"
- Debugging: "Why is the CLI slow?"
- Planning: "Which commands can I safely run right now?"

## Synopsis

```bash
prctrl health [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--json` | Output as JSON | `false` |
| `-S, --suggest` | Show actionable recommendations based on rate limits | `false` |

## Examples

```bash
# Basic health check
prctrl health

# With actionable recommendations
prctrl health --suggest
prctrl health -s

# JSON output for scripting
prctrl health --json
```

## Output Details

### Rate Limit Display

The health command shows all GitHub API rate limits with visual indicators:

- 🟢 **Green**: More than 50% remaining
- 🟠 **Yellow**: 10-50% remaining
- 🟡 **Yellow**: Less than 10% remaining  
- 🔴 **Red**: Exhausted (0 remaining)

### API Cost Table (--suggest)

When using `--suggest`, a table shows approximate API costs per command:

```
  📊 Command API Costs (approximate GitHub API calls):

  Command                    Calls   Notes
  ────────────────────────────────────────────────────────────────
  🟢 list                      ~1   per repo (list PRs)
  🟢 list --all                ~3   all repos + details
  🟢 delegate --dry-run        ~3   preview without CLAUDE
  🟡 delegate                  ~8   +PR details +CLAUDE API
  🔴 activity                  ~8   all repos + timelines
```

Color indicators show affordability based on remaining quota:
- 🟢 **Green**: Safe to run (remaining >= 10 calls)
- 🟡 **Yellow**: Use with caution (remaining >= command cost)
- 🔴 **Red**: Avoid until reset (remaining < command cost)

## Tips

- Commands with `--dry-run` cost ~3x less (skip external API calls like CLAUDE)
- Batch commands (`--all`) multiply API costs by number of PRs
- Use `--json` to reduce output parsing overhead
- When rate limits are low, prefer `summary` over `stats` (fewer API calls)
