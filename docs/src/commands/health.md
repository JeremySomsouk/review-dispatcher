# health

**Show GitHub API health status and rate limits.**

Avoid hitting rate limits by checking before large operations.

## When to Use

- Before batch operations: "Will I hit the limit?"
- Debugging: "Why is the CLI slow?"

## Synopsis

```bash
review-dispatcher health [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--json` | Output as JSON | `false` |
| `-s, --suggest` | Show actionable recommendations based on rate limits | `false` |

## Examples

```bash
# Basic health check
review-dispatcher health

# With actionable recommendations
review-dispatcher health --suggest
review-dispatcher health -s

# JSON output for scripting
review-dispatcher health --json
```
