# clean

**Remove all past review files from the output directory.**

Keep your reviews folder tidy by purging old output.

## When to Use

- Weekly cleanup: "Fresh start"
- Before `report`: "Start clean for new report period"

## Synopsis

```bash
prctrl clean
prctrl clean --dry-run
prctrl clean -n          # short form for dry-run
prctrl clean --quiet    # suppress output
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-n, --dry-run` | Preview what would be deleted without actually deleting | `false` |
| `-q, --quiet` | Suppress all output except errors | `false` |

## Tips

- Only deletes files in your local output directory
- Does not affect GitHub data
- Always use `--dry-run` first if you're unsure what will be deleted
