# clean

**Remove all past review files from the output directory.**

Keep your reviews folder tidy by purging old output.

## When to Use

- Weekly cleanup: "Fresh start"
- Before `report`: "Start clean for new report period"

## Synopsis

```bash
review-dispatcher clean
review-dispatcher clean --dry-run
review-dispatcher clean -n   # short form
```

## Options

`-n, --dry-run`
: Preview what would be deleted without actually deleting files

## Tips

- Only deletes files in your local output directory
- Does not affect GitHub data
- Always use `--dry-run` first if you're unsure what will be deleted
