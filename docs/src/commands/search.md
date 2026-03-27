# search

**Find PRs by keyword in the title.**

Quick way to locate a specific PR when you remember part of the title but not the number.

## When to Use

- "Wasn't there a PR about CSV export?"
- Finding PRs related to a specific feature
- Checking if someone's already working on something

## Synopsis

```bash
review-dispatcher search <KEYWORD> [OPTIONS]
```

## Arguments

| Argument | Description |
|----------|-------------|
| `KEYWORD` | Word or phrase to search for in PR titles (case-insensitive) |

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-P, --priority` | Show priority scores | `false` |
| `--json` | Output as JSON | `false` |

## Examples

```bash
# Find PRs with "security" in the title
review-dispatcher search security

# Find PRs about authentication
review-dispatcher search auth --priority

# Use in scripts
review-dispatcher search csv --json | jq 'length'
# Output: 2  (there are 2 PRs mentioning csv)
```

## Output Example

```
🔍 Search: "auth" (3 results)

[1] feat: add OAuth2 authentication  #4821 (backend)  👤 alice  +156  1 day ⭐⭐
[2] fix: auth token refresh         #4520 (api)     👤 bob    +45   3 days
[3] docs: update auth README        #3891 (docs)    👤 carol  +12   5 days
```

## Tips

- Search is partial-match (finds "auth" in "OAuth2" or "authentication")
- Add `--priority` to see which results are most urgent
