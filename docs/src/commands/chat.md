# chat

**Start an interactive chat session with Claude Code about your PRs.**

Get help understanding PRs, choosing which to review, or learning about PRCtrl commands.

## When to Use

- Unclear about a PR: "What does this PR do?"
- Choosing what to review: "Which PR should I look at first?"
- Learning PRCtrl: "How do I delegate reviews?"

## Prerequisites

- [Claude Code CLI](https://docs.anthropic.com/claude-cli) must be installed
- Claude Code must be authenticated (`claude /login`)

## Synopsis

```bash
prctrl chat [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-p, --pr <PR>` | Start chat about a specific PR | None |

## Examples

```bash
# Start a general chat about PRs
prctrl chat

# Chat about a specific PR
prctrl chat --pr 123
```

## How It Works

The chat command:

1. Launches Claude Code with a custom system prompt containing PRCtrl documentation
2. Provides Claude with context about your available commands
3. Lets Claude suggest relevant `prctrl` commands based on your questions

Claude will remember your PR context within the session and can help with tasks like:
- Explaining PRs in plain language
- Recommending which PRs to review first
- Suggesting delegation strategies
- Navigating complex PRs

## Tips

- Type `exit` or `quit` to end the session
- Be specific in your questions for better recommendations
- Claude knows all your pending PRs from the review queue
- Use `--pr` to get context-specific help about a particular PR
