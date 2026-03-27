# approve

**Approve a PR directly from the terminal.**

No more switching to GitHub UI for simple approvals. Approve and add a comment in one command.

## When to Use

- Code looks good after review
- Small PR you trust the author on
- Quick approval to unblock CI

## Synopsis

```bash
review-dispatcher approve [OPTIONS]
```

## Options

| Flag | Description |
|------|-------------|
| `-p, --pr <NUM>` | PR number to approve |
| `-m, --message <TEXT>` | Approval comment (optional) |

## Examples

```bash
# Approve with default message
review-dispatcher approve --pr 4821

# Approve with a comment
review-dispatcher approve --pr 4821 -m "LGTM! Nice work on the tests."

# Approve without comment
review-dispatcher approve --pr 4821 -m ""
```

## Tips

- Requires PR to already be reviewed
- Use `--pr` flag or positional argument
