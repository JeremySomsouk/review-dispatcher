# config

**Manage PRCtrl configuration settings.**

Create, view, and update your local configuration file.

## When to Use

- Initial setup: `prctrl config init`
- Check current config: `prctrl config show`
- Update specific fields: `prctrl config update --token xxx`

## Configuration File Location

The configuration file is stored at: `~/.prctrl/config.toml`

## Synopsis

```bash
# Interactive configuration setup
prctrl config init
prctrl config init --force   # Overwrite existing without prompting

# Display current configuration
prctrl config show

# Update specific configuration fields
prctrl config update --token YOUR_GITHUB_TOKEN
prctrl config update --org your-org --repos owner/repo1,owner/repo2
prctrl config update --teams team-slug-1,team-slug-2
```

## Subcommands

### init

Create a new configuration file interactively.

```bash
prctrl config init
prctrl config init --force   # Overwrite without prompting
```

You'll be prompted for:
- GitHub Token (required)
- GitHub Username (required)
- GitHub Organization (required)
- GitHub Repositories (optional, comma-separated)
- GitHub Teams (optional, comma-separated slugs)
- Crew Members (optional, comma-separated usernames)
- Anthropic API Key (optional)

### show

Display the current configuration file contents.

```bash
prctrl config show
```

### update

Update specific configuration fields without full re-initialization.

```bash
prctrl config update --token YOUR_TOKEN
prctrl config update --org your-org
prctrl config update --repos owner/repo1,owner/repo2
prctrl config update --teams team-slug-1,team-slug-2
prctrl config update --crew username1,username2
prctrl config update --api-key YOUR_API_KEY
```

All flags are optional. Only provided flags will be updated.

## Options

### init Options

| Flag | Description | Default |
|------|-------------|---------|
| `-f, --force` | Overwrite existing configuration without prompting | `false` |

### show Options

No additional options.

### update Options

| Flag | Description |
|------|-------------|
| `--token` | GitHub personal access token |
| `--username` | GitHub username |
| `--org` | GitHub organization |
| `--repos` | Comma-separated list of repositories (owner/repo format) |
| `--teams` | Comma-separated list of GitHub team slugs |
| `--crew` | Comma-separated list of crew member usernames |
| `--api-key` | Anthropic API key |

## Examples

### Initial Setup

```bash
$ prctrl config init
📝 Creating new configuration file...
   Path: /home/user/.prctrl/config.toml

GitHub Token (RD_GITHUB_TOKEN or GITHUB_TOKEN): ghp_xxxxxxxxxxxx
GitHub Username (RD_GITHUB_USERNAME or GITHUB_USERNAME): myusername
GitHub Organization (RD_GITHUB_ORG or GITHUB_ORG): my-org
GitHub Repositories (comma-separated, e.g. owner/repo1,owner/repo2) (optional): owner/repo1,owner/repo2
GitHub Teams (comma-separated slugs, optional): my-team,other-team
Crew Members (comma-separated usernames, optional): alice,bob
Anthropic API Key (optional):

✅ Configuration saved successfully!
```

### Check Current Config

```bash
$ prctrl config show
📄 Configuration File: /home/user/.prctrl/config.toml

--------------------------------------------------
# PRCtrl Configuration File
# Created at: 2024-01-15T10:30:00+00:00

[github]
token = "ghp_xxxxxxxxxxxx"
username = "myusername"
org = "my-org"
repos = ["owner/repo1", "owner/repo2"]
teams = ["my-team", "other-team"]
crew_members = ["alice", "bob"]
--------------------------------------------------
```

### Update Specific Fields

```bash
$ prctrl config update --org new-org --teams new-team
✅ Configuration updated successfully!
   Path: /home/user/.prctrl/config.toml
```

## Notes

- The configuration file is stored in TOML format
- Environment variables (`RD_*` or `GITHUB_*`) take precedence over config file values
- Use `prctrl config init --force` to regenerate the entire config file
