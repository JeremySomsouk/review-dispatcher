# monitor

**Watch for new PRs and get notified — runs in the background.**

Perfect for developers who want to know immediately when their attention is needed, without constantly checking manually.

## When to Use

- You want instant notifications for new PRs
- You're in a flow state and don't want to switch contexts
- You need to catch PRs that request your review ASAP

## Synopsis

```bash
prctrl monitor [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-i, --interval <SECONDS>` | How often to check GitHub | `300` (5 min) |
| `--notify` | Send macOS notifications for new PRs | `false` |
| `--auto-open` | Auto-open PR in Chrome when notified | `false` |
| `--no-auto-open` | Disable auto-opening (use with `--notify`) | `false` |
| `--interactive` | Prompt for action on each new PR | `false` |

**Note:** `--notify` and `--auto-open` are disabled by default. Use `--notify` to enable notifications, and `--auto-open` (or `--notify --auto-open`) to also open PRs automatically.

## Examples

```bash
# Start monitoring with defaults (checks every 5 minutes, no notifications)
prctrl monitor

# Check more frequently (every minute)
prctrl monitor --interval 60

# Enable notifications (no auto-open)
prctrl monitor --notify

# Enable notifications AND auto-open in Chrome
prctrl monitor --notify --auto-open

# Notifications only, no auto-open (good for meetings)
prctrl monitor --notify --no-auto-open

# Interactive mode - choose what to do for each new PR
prctrl monitor --notify --interactive

# Run in background (add & to detach)
prctrl monitor &
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
prctrl monitor &

# Check if running
prctrl monitor-status

# Stop monitoring
prctrl monitor-stop
```

## Tips

- Use `--interval 60` during code freeze/release when PRs pile up
- Combine with `--no-auto-open` when you need to context-switch carefully
- The monitor process survives terminal restarts (uses a PID file)
