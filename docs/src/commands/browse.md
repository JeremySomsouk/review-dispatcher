# browse

**Open one or more PRs in your browser.**

Jump straight to GitHub without switching windows or copying URLs.

## When to Use

- After triage: "Let me see the actual code"
- Quick access: "Open all my pending PRs"

## Synopsis

```bash
review-dispatcher browse [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to open (shorthand for `--pr`) | - |
| `-p, --pr <NUM>` | Open specific PR (shorthand for `--pr`) | - |
| `-n, --pr-numbers <NUMS>` | PR number(s) to open (comma-separated) | - |
| `-a, --all` | Open all pending reviews | `false` |
| `--json` | Output URLs as JSON (without opening browser) | `false` |

**Note:** The global `--pr` flag (`-p`) also works with this command for consistency with other commands.

## Examples

```bash
# Open a specific PR in browser
review-dispatcher browse 4821
review-dispatcher browse --pr 4821

# Open multiple PRs in browser
review-dispatcher browse -n 4821,3156,2890

# Open all pending reviews at once
review-dispatcher browse --all

# Output URLs as JSON (useful for scripting)
review-dispatcher browse -p 4821,3156 --json
```
