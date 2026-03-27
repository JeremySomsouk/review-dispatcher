# monitor

Monitor for new PRs and send macOS notifications.

## Synopsis

```bash
review-dispatcher monitor [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-i, --interval <SECONDS>` | Polling interval in seconds | 300 (5 min) |
| `-n, --notify` | Send macOS notifications | true |
| `--auto-open` | Auto-open PRs in Chrome | true |
| `--no-auto-open` | Disable auto-open | - |
| `--interactive` | Interactive mode with prompts | false |

## Examples

```bash
# Monitor with defaults (5 min interval, notifications on)
review-dispatcher monitor

# Fast polling (1 minute)
review-dispatcher monitor --interval 60

# No auto-open (just notifications)
review-dispatcher monitor --no-auto-open

# Interactive mode
review-dispatcher monitor --interactive
```

## Background Mode

Run in background with `&`:

```bash
review-dispatcher monitor &
```

## Related Commands

- `monitor-stop` - Stop the monitor
- `monitor-status` - Check if running
