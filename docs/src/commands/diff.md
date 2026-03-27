# diff

Show detailed diff/stats for a specific PR directly in the terminal.

## Synopsis

```bash
review-dispatcher diff [OPTIONS]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--pr, -p <NUMBER>` | Target a specific PR by number | Interactive selection |

## Examples

```bash
# Show diff for a specific PR
review-dispatcher diff --pr 4821

# Interactive mode (select from pending reviews)
review-dispatcher diff
```

## Output

Displays comprehensive PR information including:

- **Title & Number**: PR title with number
- **Author & Age**: Who opened it and when
- **Branch**: Source branch name
- **Status**: DRAFT or READY status
- **Lines**: Additions and deletions
- **Size Category**: XS (<50), S (50-200), M (200-500), L (500-1000), XL (1000+)
- **Age Category**: 🔥 HOT (today), ⚡ FRESH (1-2d), 📅 WEEK OLD (3-7d), ⚠️ STALE (8-14d), 🚨 OLD (15d+)
- **Priority Score**: 1-5 star rating based on urgency
- **Repository**: Full repo name
- **URL**: Direct link to the PR

```
📄 feat: add CSV export  #4821
   👤 sarah_dev  •  📅 5 days ago  •  🌿 feature/export
   📊 READY  •  +245 additions  •  -32 deletions
   🔗 https://github.com/org/repo/pull/4821
────────────────────────────────────────────────────
   📦 Size: M (277 lines)
   ⏱️  Age: 📅 WEEK OLD (5 days)
   ⭐ Priority: 3/5  ⭐⭐⭐
   📁 Repository: myorg/frontend
────────────────────────────────────────────────────
```
