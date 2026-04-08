# stack

**Detect and visualize stacked PRs (sequential dependencies).**

## When to Use

- Review planning: "Which PRs depend on each other?"
- Merge ordering: "What needs to merge first?"

## Synopsis

```bash
prctrl stack [flags]
```

## Detection Methods

PRCtrl uses two methods to detect stacked PRs:

**1. Branch chaining** — A PR's base branch is another PR's head branch:
- PR A: `feature` → `main`
- PR B: `feature-2` → `feature`
- This creates a stack: PR A → PR B

**2. Convention** — PRs sharing the same ticket key with `[N/M]` position markers:
- PR #1: `refactor(TAHC-1666): add client [1/3]` on branch `TAHC-1666-client`
- PR #2: `refactor(TAHC-1666): add service [2/3]` on branch `TAHC-1666-service`
- PR #3: `refactor(TAHC-1666): add tests [3/3]` on branch `TAHC-1666-tests`
- These are grouped into a stack by the `TAHC-1666` ticket key

## Flags

- `--json` — Output as JSON (useful for scripting)
- `-r, --repo <REPO>` — Filter by repository name (partial match, case-insensitive)
- `--author <AUTHOR>` — Filter by author username
- `-n, --limit <N>` — Limit number of stacks to show

## Examples

```bash
# Show all stacked PRs across configured repos
prctrl stack

# Filter by repository
prctrl stack --repo health-content

# Filter by author
prctrl stack --author JeremySomsouk

# Get JSON output for scripting
prctrl stack --json

# Limit to 5 stacks
prctrl stack -n 5
```

## See Also

- [mine](mine.md) — Shows your own PRs with automatic stack detection
- [list](list.md) — Use `--show-stacks` to display stacks with pending reviews
