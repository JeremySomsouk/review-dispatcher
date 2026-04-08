# mine

**List your own open PRs — draft or not.**

Track your own pull requests from the command line.

## When to Use

- Status check: "What's my open PRs?"
- Draft management: "Find all my draft PRs"

## Synopsis

```bash
prctrl mine [flags]
```

## Flags

- `--json` — Output as JSON (useful for scripting)
- `-a, --all` — Show all matching PRs at once (without prompting)
- `-P, --priority` — Show priority score (1-5 stars) based on age and size
- `-s, --since-days <DAYS>` — Only show PRs created since this many days ago
- `--repo <REPO>` — Filter by repository name (partial match, case-insensitive)
- `--author <AUTHOR>` — Filter by author username (partial match, case-insensitive)
- `-p, --pr <PR_NUMBER>` — Target a specific PR by number
- `--pr-numbers <NUMBERS>` — Target multiple PRs by number (comma-separated, e.g., `123,456,789`)

## Stack Detection

The `mine` command automatically detects and displays **stacked PRs** — PRs that build on each other. Stacks are shown after your individual PR list.

### Detection methods

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

Example stack output:
```
┌─ Stack on `main` (3 PRs, branch-chain)

🔵 [1/3] #974 - refactor(TAHC-1666): extract OnboardingCmsClient port [1/3] [DRAFT]
  └─ TAHC-1666-onboarding-cms-client-port
    https://github.com/doctolib/health-content/pull/974

  [2/3] #975 - refactor(TAHC-1666): extract OnboardingCompletionChecker port [2/3] [DRAFT]
  └─ TAHC-1666-onboarding-completion-checker
    https://github.com/doctolib/health-content/pull/975

  [3/3] #976 - refactor(TAHC-1666): extract OnboardingDebugController [3/3] [DRAFT]
  └─ TAHC-1666-onboarding-debug-controller
    https://github.com/doctolib/health-content/pull/976
```

## Global Flags

These flags are available globally and work with `mine`:

- `-p, --pr <PR_NUMBER>` — Target a specific PR by number (overrides filters)
- `-d, --include-drafts` — Include draft PRs in results
- `--exclude-prefix <PREFIXES>` — Exclude PRs with matching title prefixes (comma-separated)
- `-o, --output-dir <PATH>` — Folder for review files (default: ./reviews)

## Snooze Behavior

Snoozed PRs are automatically hidden from `mine` results (consistent with `list` and `delegate`). Use `--pr`, `--pr-number`, or `--pr-numbers` to bypass this filter and view specific snoozed PRs.

## Examples

```bash
# List your open PRs
prctrl mine

# Show all PRs at once (non-interactive)
prctrl mine --all

# Show your PRs with priority scores
prctrl mine --priority

# Get JSON output for scripting
prctrl mine --json

# Include draft PRs
prctrl mine -d

# Only show PRs from the last 7 days
prctrl mine --since-days 7

# Filter by repository
prctrl mine --repo my-repo

# Filter by author
prctrl mine --author johndoe

# Target a specific PR (bypasses snooze filter)
prctrl mine --pr 123

# Target multiple specific PRs
prctrl mine --pr-numbers 123,456,789

# Combine filters
prctrl mine --since-days 14 --repo api --priority

# Combine with global flags
prctrl mine --include-drafts --priority
```
