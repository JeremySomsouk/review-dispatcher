# files

**Show changed files for one or more PRs.**

Quickly see which files were modified before diving into the diff.

## When to Use

- Pre-review scan: "What did they change?"
- Impact assessment: "Which services are affected?"

## Synopsis

```bash
review-dispatcher files [OPTIONS] [PR_NUMBER]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `PR_NUMBER` | PR number to show files for (shorthand for `--pr`) | - |
| `-p, --pr <PR>` | Show files for specific PR (shorthand) | - |
| `-n, --pr-numbers <NUMS>` | PR number(s) to show files for (comma-separated) | - |
| `-a, --all` | Show files for all pending reviews | `false` |
| `--json` | Output as JSON (useful for scripting) | `false` |

## Examples

```bash
review-dispatcher files 4821
review-dispatcher files --pr 4821
review-dispatcher files -n 4821,3156,2890
review-dispatcher files --all
review-dispatcher files --pr 4821 --json
```

## JSON Output

When `--json` is specified, output includes for each PR:

- `pr_number`, `pr_title`, `repo`, `url`
- `total_files`, `total_additions`, `total_deletions`
- `files[]` - array of file objects with `filename`, `status`, `additions`, `deletions`
