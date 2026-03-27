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

## Examples

```bash
review-dispatcher health
```
