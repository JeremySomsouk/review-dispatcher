# Troubleshooting PRCtrl

This guide covers common issues and their solutions.

## Quick Fixes

### "No pending reviews" but you know there are PRs

1. **Check your configuration**
   ```bash
   prctrl config show
   ```

2. **Verify GitHub token has correct permissions**
   - Go to GitHub → Settings → Developer Settings → Personal Access Tokens
   - Ensure token has `repo` scope (full control of repositories)

3. **Check you're listed as a reviewer**
   ```bash
   # See your username in the config
   prctrl config show
   
   # List PRs including your own
   prctrl list --include-mine
   ```

4. **Check repository access**
   ```bash
   # Ensure repos are configured correctly
   prctrl config show
   
   # Try specifying repo explicitly
   prctrl list --repo <repo-name>
   ```

### "Set PRCTRL_GITHUB_TOKEN" error

**Cause:** No GitHub token configured.

**Solution:**
```bash
# Option 1: Interactive setup
prctrl config init

# Option 2: Environment variable
export PRCTRL_GITHUB_TOKEN=ghp_xxxxxxxxxxxx
export PRCTRL_GITHUB_USERNAME=your_username
export PRCTRL_GITHUB_ORG=your_org

# Option 3: Add to config file (~/.prctrl/config.toml)
```

See [Configuration](https://jeremysomsouk.github.io/prctrl/#configuration) for details.

### Authentication errors

1. **Token may have expired**
   ```bash
   # Regenerate token on GitHub and update
   prctrl config update --token <new-token>
   ```

2. **Token scope insufficient**
   - Required scopes: `repo`, `read:user`, `notifications`

3. **Wrong username in config**
   ```bash
   prctrl config update --username <your-github-username>
   ```

### Rate limiting

**Symptom:** `GitHub API rate limit exceeded` error

**Solutions:**
```bash
# Check rate limit status
prctrl health

# Wait for reset (usually 1 hour)

# Consider using a GitHub OAuth token for higher limits
# (Tokens from GitHub Apps have higher rate limits)
```

### Monitor not working

1. **Check if monitor is running**
   ```bash
   prctrl monitor-status
   ```

2. **Stop and restart**
   ```bash
   prctrl monitor-stop
   prctrl monitor
   ```

3. **Check macOS notifications permissions**
   - System Preferences → Notifications → Allow notifications from Terminal

### Claude integration not working

1. **Check API key is set**
   ```bash
   # Environment variable
   export PRCTRL_ANTHROPIC_API_KEY=sk-ant-xxxxx
   
   # Or in config update
   prctrl config update --api-key <key>
   ```

2. **Verify Claude CLI is installed**
   ```bash
   which claude
   claude --version
   ```

3. **Check delegation command works**
   ```bash
   prctrl delegate --dry-run
   ```

### Empty output or missing data

1. **Check filters**
   ```bash
   # List all (remove --crew filter)
   prctrl list
   
   # Include drafts
   prctrl list --include-drafts
   
   # Include your own PRs
   prctrl list --include-mine
   ```

2. **Check exclude prefixes**
   ```bash
   # Default excludes "chore(deps)" - check your PR titles
   prctrl list --exclude-prefix ""
   ```

3. **Check output directory permissions**
   ```bash
   ls -la ./reviews
   # or
   ls -la ~/Library/Application\ Support/prctrl/reviews
   ```

### Build errors

**Missing dependencies:**
```bash
# Update Rust
rustup update

# Fetch dependencies
cargo fetch
```

**Old Rust version:**
```bash
# Check version
rustc --version

# Update if below 1.70
rustup update
```

## Configuration Issues

### Config file location

PRCtrl looks for config in this order:
1. Environment variables
2. `~/.prctrl/config.toml`
3. `~/Library/Application Support/prctrl/config.toml` (macOS)

### Verify config is loaded

```bash
prctrl config show
```

### Reset configuration

```bash
# Remove existing config
rm ~/.prctrl/config.toml

# Re-initialize
prctrl config init --force
```

## Performance Issues

### Slow command execution

1. **Many repositories configured**
   - Reduce `repos` list to only active ones

2. **Large number of PRs**
   - Use filters: `--repo`, `--author`, `--since-days`

3. **Network latency**
   - Check internet connection
   - GitHub API has inherent latency

### High memory usage

- Close other applications
- Reduce concurrent operations

## macOS Specific

### Notifications not appearing

1. **Check notification permissions**
   ```
   System Preferences → Notifications → Find Terminal/App in list
   ```

2. **Check Focus/Do Not Disturb**
   - Temporarily disable to test notifications

3. **Check monitor is actually running**
   ```bash
   prctrl monitor-status
   ```

### Can't find config file

Config is stored at:
- `~/.prctrl/config.toml` (preferred)
- `~/Library/Application Support/prctrl/config.toml`

```bash
# Find it
find ~ -name "config.toml" -path "*prctrl*"
```

## Getting Help

If you're still stuck:

1. **Check existing issues**
   - https://github.com/JeremySomsouk/prctrl/issues

2. **Run with verbose output**
   ```bash
   RUST_LOG=debug cargo run -- list
   ```

3. **Report an issue** with:
   - Error message
   - Command you ran
   - Output of `prctrl config show` (remove sensitive data)
   - Your PRCtrl version: `cargo run -- --version`
