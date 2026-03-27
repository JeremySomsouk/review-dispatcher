# monitor

**Watch for new PRs and get notified — runs in the background.**

Perfect for developers who want to know immediately when their attention is needed, without constantly checking manually.

## When to Use

- You want instant notifications for new PRs
- You're in a flow state and don't want to switch contexts
- You need to catch PRs that request your review ASAP

## Synopsis

```bash
review-dispatcher monitor [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-i, --interval <SECONDS>` | How often to check GitHub | `300` (5 min) |
| `-n, --notify` | Send macOS notifications | `true` |
| `--auto-open` | Auto-open PR in Chrome when notified | `true` |
| `--no-auto-open` | Notifications only, no browser | - |
| `--interactive` | Prompt for action on each new PR | `false` |

## Examples

```bash
# Start monitoring with defaults (checks every 5 minutes)
review-dispatcher monitor

# Check more frequently (every minute)
review-dispatcher monitor --interval 60

# Notification only, no auto-open (good for meetings)
review-dispatcher monitor --no-auto-open

# Interactive mode - choose what to do for each new PR
review-dispatcher monitor --interactive

# Run in background (add & to detach)
review-dispatcher monitor &
```

## Interactive Mode Actions

When `--interactive` is enabled, each new PR shows:

```
🔔 New PR: feat: add dark mode #4821

🎯 What to do?
  [d] Delegate to Claude
  [o] Open in browser
  [a] Assign myself
  [s] Snooze (hide for 3 days)
  [q] Quit monitoring
```

## Background Operation

```bash
# Start and detach
review-dispatcher monitor &

# Check if running
review-dispatcher monitor-status

# Stop monitoring
review-dispatcher monitor-stop
```

## Tips

- Use `--interval 60` during code freeze/release when PRs pile up
- Combine with `--no-auto-open` when you need to context-switch carefully
- The monitor process survives terminal restarts (uses a PID file)
