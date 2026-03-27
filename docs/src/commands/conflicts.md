# conflicts

**Show which pending PRs have merge conflicts.**

Avoid wasting time on PRs that can't be merged yet.

## When to Use

- Pre-review: "Is this even mergeable?"
- Sprint planning: "Which PRs are blocked?"
- Merge day: "What can we actually ship?"

## Synopsis

```bash
review-dispatcher conflicts [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--conflicts-only` | Hide PRs without conflicts | `false` |
| `--json` | Output as JSON | `false` |

## Examples

```bash
review-dispatcher conflicts
review-dispatcher conflicts --conflicts-only
```
