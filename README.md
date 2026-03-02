# review-dispatcher

**Monitor GitHub PRs, get notified about new ones, and delegate reviews to Claude, all from your terminal.**

## 🎯 Quick Start (2 minutes)

```bash
# 1. Install
cargo install --git https://github.com/JeremySomsouk/review-dispatcher

# 2. Configure
cp .env.example .env
# Edit .env with your GitHub token

# 3. Monitor for new PRs
review-dispatcher monitor
```

**That's it!** You'll get desktop notifications when new PRs arrive.

## 🔔 Notification Features

The monitor includes powerful macOS notification features:

### Enhanced Notifications
- **🦀 Rust Crab Emoji** - Easily recognizable notifications
- **Custom Subtitle** - "Review Dispatcher" context
- **Glass Sound** - Pleasant notification chime
- **PR Details** - Shows PR number, title, author, and repo

### Auto-Open Browser (Default Behavior)
By default, when a notification appears:
- **Notification pops up** with PR details
- **Chrome automatically opens** to the PR URL
- **No clicking needed** - immediate access to the PR

### Disable Auto-Open
To disable auto-opening and just show notifications with URLs:

```bash
# Show notification with URL (no auto-open)
# This feature will be available in a future update
review-dispatcher monitor --no-auto-open
```

For now, auto-open is always enabled. The notification will show the PR details and Chrome will open automatically.

### Notification Content
Notifications include:
```
🦀 New PR: #123
Review Dispatcher
Fix login bug by alice in frontend-app
```

### Troubleshooting Notifications
If notifications aren't working:
1. Check macOS notification permissions for your terminal
2. Test with: `osascript -e 'display notification "Test" with title "Review Dispatcher"'`
3. Ensure "Do Not Disturb" is disabled
4. Check terminal-specific notification settings

See the [Troubleshooting](#-troubleshooting) section for detailed help.

## Installation

```bash
git clone git@github.com:JeremySomsouk/review-dispatcher.git
cd review-dispatcher
cargo install --path .
```

This installs the `review-dispatcher` binary to `~/.cargo/bin/`, making it available globally.

For development, use `cargo run --` instead:

```bash
cargo run -- list
```

## Configuration

Copy the example env file and fill in your values:

```bash
cp .env.example .env
```

```env
RD_GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxx       # Required — GitHub personal access token
RD_GITHUB_USERNAME=JeremySomsouk               # Required — your GitHub login
RD_GITHUB_ORG=my-org                             # Required — GitHub org to scan
RD_GITHUB_REPOS=frontend-app,backend-api         # Required — comma-separated repo list
RD_GITHUB_TEAMS=my-team                          # Optional — comma-separated team slugs to match
RD_CREW_MEMBERS=alice,bob,charlie               # Optional — GitHub logins for --crew filter
```

### Custom Review Instructions

Create an `instruction.md` file to customize Claude's review criteria. The program searches in multiple locations:

**Search Order:**
1. **Command line flag**: `--instruction-path /path/to/your/instructions.md`
2. **Environment variable**: `RD_INSTRUCTION_PATH=/path/to/instructions.md`
3. **Current directory**: `./instruction.md`
4. **User config directory**: `~/.review-dispatcher/instruction.md`
5. **Fallback**: Built-in default instructions

**Setup:**
```bash
# Option 1: Use current directory (project-specific)
cp instruction.md.example instruction.md
nano instruction.md

# Option 2: Use user config directory (global)
mkdir -p ~/.review-dispatcher
cp instruction.md.example ~/.review-dispatcher/instruction.md
nano ~/.review-dispatcher/instruction.md

# Option 3: Use environment variable (flexible)
export RD_INSTRUCTION_PATH="/custom/path/instructions.md"
review-dispatcher delegate

# Option 4: Use command line flag (override)
review-dispatcher delegate --instruction-path /custom/instructions.md
```

The `instruction.md` file allows you to:
- Define team-specific review criteria
- Set consistent evaluation standards
- Add custom focus areas (security, performance, etc.)
- Provide context about your codebase
- Specify review priorities

When present, these instructions are automatically included in every Claude review prompt, ensuring consistent evaluation across all PRs.

### GitHub Token

Create a [fine-grained personal access token](https://github.com/settings/tokens?type=beta) with **Pull requests: Read** access on your target repos.

**Important**: This tool uses the GitHub API directly (via `octocrab`), so it works from any directory — **no git repository required**. You don't need to run `git init` or be inside a git repo.

### Team Matching

Set `GITHUB_TEAMS` to match PRs where your team is a requested reviewer (not just you personally). Use the team's **slug** (lowercase, hyphenated). Multiple teams are comma-separated:

```env
GITHUB_TEAMS=my-team,another-team
```

### Claude CLI

The `delegate` command uses the [Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code) (`claude --print`) to triage PRs. Make sure you're logged in:

```bash
claude --version   # verify installation
```

**Note**: This tool does NOT require a git repository or use `git` commands. It uses the GitHub API directly via `octocrab`, so you can run it from any directory. If Claude asks about `git init`, you can safely decline — the tool works without git repository access.

No API key needed — it uses your existing Claude Code session.

## Usage

### Quick start

```bash
# See what's waiting for you
review-dispatcher list

# Include your own PRs and drafts
review-dispatcher -m -d list

# Only show PRs from your crew
review-dispatcher -c list

# Let Claude triage everything
review-dispatcher delegate

# Triage one specific PR
review-dispatcher delegate 680

# Clean up past review files
review-dispatcher clean

# Monitor for new PRs with macOS notifications
review-dispatcher monitor
```

### List pending reviews

Fetch all PRs where you (or your team) are a requested reviewer, print them to the terminal, and write Markdown files:

```bash
review-dispatcher list
```

### List your own open PRs

Show all your open PRs (including drafts if `-d` flag is used):

```bash
review-dispatcher mine
review-dispatcher mine -d      # Include draft PRs
```

Output:

```
🔍  3 pending review(s) assigned to you

  [1]  feat: add CSV export  #4821 (frontend-app)
      👤 alice  •  +120 -30 lines  •  opened 2 days ago
      🌿 alice:feat/csv-export  🔗 https://github.com/my-org/frontend-app/pull/4821

  [2]  fix: flaky CI on scheduling  #312 (backend-api)
      👤 bob  •  +8 -3 lines  •  opened 5 days ago
      🌿 bob:fix/flaky-ci  🔗 https://github.com/my-org/backend-api/pull/312

📁 Written to ./reviews  (index: INDEX.md)
```

### Delegate reviews to Claude

Ask Claude to triage each pending review with a summary, recommendation:

```bash
review-dispatcher delegate
```

Output includes:
- PR summary and key review areas
- Recommendation (REVIEW NOW/DELEGATE)

Example:
```
────────────────────────────────────────────────────────
🤖 Claude on:  feat: add CSV export  #4821

CUSTOM REVIEW INSTRUCTIONS:
- Security First: Always check for potential security vulnerabilities
- Performance Matters: Look for N+1 queries

This PR adds CSV export functionality with proper validation.
Key areas: data privacy, performance optimization.

Recommendation: [REVIEW NOW] — security-critical changes
────────────────────────────────────────────────────────
   💾 Saved → ./reviews/frontend-app-4821-feat-add-csv-export.md
```

### Delegate a specific PR

```bash
review-dispatcher delegate 4821
# or equivalently:
review-dispatcher -p 4821 delegate
```

### Custom output directory

```bash
review-dispatcher --output-dir ~/work/reviews list
review-dispatcher --output-dir ~/work/reviews delegate
```

### Exclude PRs by title prefix

PRs with titles starting with `chore(deps)` are excluded by default (e.g. Renovate/Dependabot). You can customize this:

```bash
# Exclude multiple prefixes
review-dispatcher --exclude-prefix "chore(deps),bot:" list

# Disable the filter entirely
review-dispatcher --exclude-prefix "" list
```

### Include your own PRs

By default, PRs you authored are excluded. Use `-m` / `--include-mine` to see them:

```bash
review-dispatcher -m list
review-dispatcher -m delegate
```

### Include draft PRs

Draft PRs are excluded by default. Use `-d` / `--include-drafts` to include them:

```bash
review-dispatcher -d list
review-dispatcher -m -d list
```

### Crew mode

Only show open PRs authored by your crew members (configured via `RD_CREW_MEMBERS` in `.env`):

```bash
review-dispatcher -c list
review-dispatcher -c delegate
```

### Target a specific PR

Use `-p` / `--pr` globally or pass the number as a positional argument to `delegate`. This bypasses review-request filters and fetches the PR directly:

```bash
review-dispatcher -p 680 list
review-dispatcher delegate 680
```

### Clean past reviews

Remove all files from the output directory:

```bash
review-dispatcher clean
review-dispatcher -o ~/work/reviews clean
```

### Monitor for new PRs

Run a background process that polls GitHub for new PRs and sends macOS notifications:

```bash
# Basic monitor (5-minute intervals, notifications on)
review-dispatcher monitor

# Crew mode with 2-minute polling
review-dispatcher monitor -c -i 120

# Monitor including drafts, no notifications
review-dispatcher monitor -d -n false

# Interactive mode - prompt for actions on new PRs
review-dispatcher monitor --interactive

# Combined: interactive + crew mode
review-dispatcher monitor -c -i 120 --interactive --notify

# Monitor with custom prefix exclusions
review-dispatcher monitor --exclude-prefix "chore,docs,test"
```

The monitor will:
- Poll GitHub at your specified interval (default: 300 seconds / 5 minutes)
- Detect new PRs that match your current filter criteria
- Send macOS notifications with PR details (title, author, repo, number)
- **Auto-open Chrome** to the PR URL when notification appears
- Print console output for each new PR detected
- Support Ctrl+C to stop monitoring
- **Prevent duplicate processes** - Only one monitor can run at a time

### Interactive Monitor Mode

When using `--interactive` flag, the monitor will prompt you for quick actions when new PRs are detected:

```bash
# Start interactive monitoring
review-dispatcher monitor --interactive

# Interactive + notifications
review-dispatcher monitor --interactive --notify
```

**Interactive mode features:**
- ✅ Quick action menu for each new PR
- ✅ Delegate to Claude with one keypress (`d`)
- ✅ Open PR in browser (`o`)
- ✅ Open review file in IntelliJ (`i`)
- ✅ Skip PRs you don't want to review (`s`)
- ✅ Temporary quit interactive mode (`q`)

**Example workflow:**
```
🔔 New PR detected: #123 - Fix login bug
✓ macOS notification sent

🎯 Quick Actions for PR #123:
  [d] Delegate to Claude for review
  [o] Open PR in browser
  [i] Open in IntelliJ
  [s] Skip this PR
  [q] Quit interactive mode

Choose action: d
⏳ Delegating PR #123 to Claude...
✅ Claude review completed:
   This PR fixes a critical login authentication issue...
   💾 Saved to ./reviews/repo-123-fix-login-bug.md
   
   Open in IntelliJ? [y/N] y
   ✅ Opening in IntelliJ IDEA...
```

### Monitor Process Management

```bash
# Check if monitor is running
review-dispatcher monitor-status

# Stop the running monitor
review-dispatcher monitor-stop

# Start monitor in background
review-dispatcher monitor &
```

**Process management features:**
- ✅ Automatic PID file management
- ✅ Prevents duplicate monitor instances
- ✅ Clean shutdown on Ctrl+C
- ✅ Status checking with `monitor-status`
- ✅ Graceful stopping with `monitor-stop`

> ⚠️  The monitor runs indefinitely until you stop it with Ctrl+C or `monitor-stop`.

### Open a terminal tab after running

Automatically opens a new iTerm2/Terminal.app tab in the output folder:

```bash
review-dispatcher --open-terminal list
review-dispatcher --output-dir ~/work/reviews --open-terminal delegate
```

## Output Files

After running, your output folder looks like:

```
reviews/
├── INDEX.md                                        ← overview table with links
├── frontend-app-4821-feat-add-csv-export.md        ← one file per PR
└── backend-api-312-fix-flaky-ci-on-scheduling.md
```

Each file contains a metadata table, PR link, and Claude's triage note (if delegated).

## Shell Aliases

Add to your `~/.zshrc` for quick access:

```bash
alias reviews="review-dispatcher list"
alias myprs="review-dispatcher mine"
alias crew="review-dispatcher -c list"
alias triage="review-dispatcher --open-terminal delegate"
alias triage-pr="review-dispatcher delegate"
alias reviews-clean="review-dispatcher clean"
alias monitor="review-dispatcher monitor"
alias monitor-stop="review-dispatcher monitor-stop"
alias monitor-status="review-dispatcher monitor-status"
alias monitor-interactive="review-dispatcher monitor --interactive"
```

Then just run:

```bash
reviews          # list all pending reviews
crew             # list crew PRs only
triage           # triage everything with Claude + open terminal
triage-pr 680    # triage a single PR
reviews-clean    # remove past review files
monitor          # monitor for new PRs with notifications
monitor-interactive # interactive monitor with quick actions
monitor-stop      # stop the running monitor process
monitor-status   # check if monitor is running
```

## CLI Reference

```
review-dispatcher [OPTIONS] <COMMAND>

Commands:
  list          List all PRs waiting for your review
  mine          List your own open PRs (draft or not)
  delegate      Ask Claude to triage each pending review
  clean         Remove all past review files from the output directory
  monitor       Monitor for new PRs with macOS notifications and interactive mode
  monitor-stop  Stop the running monitor process
  monitor-status Check if monitor process is running

Global options:
  -o, --output-dir <PATH>     Folder for review files (default: ./reviews)
      --open-terminal          Open a new terminal tab in output dir after running
  -m, --include-mine           Include PRs you authored
  -d, --include-drafts         Include draft PRs
  -c, --crew                   Only show open PRs from crew members
  -p, --pr <NUMBER>            Target a specific PR (bypasses review-request filters)
      --exclude-prefix <P>     Exclude PRs by title prefix (default: "chore(deps)")
      --instruction-path <P>   Path to custom instruction file
  -h, --help                   Print help

Delegate accepts a positional PR number:
  review-dispatcher delegate 680    # shorthand for -p 680 delegate

Monitor-specific options:
  -i, --interval <SECONDS>     Polling interval in seconds (default: 300)
  -n, --notify                 Send macOS notifications for new PRs (default: true)
      --interactive            Enable interactive mode with quick actions
```

## Requirements

- Rust 1.70+
- A GitHub personal access token with PR read access
- [Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code) (for the `delegate` command)
- macOS (for the `monitor --notify` feature)

## 📚 Summary

review-dispatcher provides a complete workflow for managing GitHub PR reviews:

1. **Monitor** - Background process watches for new PRs
2. **Notify** - macOS notifications alert you to new PRs
3. **Interact** - Quick action menu lets you choose what to do
4. **Delegate** - Claude provides automated triage and summaries
5. **Review** - Open PRs in browser or IntelliJ with one command

Perfect for engineering managers, tech leads, and developers who want to stay on top of their team's PRs without constant manual checking.

## 🔮 Future Enhancements

Potential features to add:
- **Slack integration** - Post notifications to Slack channels
- **PR prioritization** - Auto-prioritize based on size, age, author
- **Review rotation** - Automatically assign reviewers
- **Metrics dashboard** - Track review times and throughput
- **Web interface** - Simple dashboard for PR management

## 🤝 Contributing

Contributions welcome! Please open issues or pull requests for:
- Bug fixes
- Feature requests
- Documentation improvements
- Performance optimizations

## 🐛 TROUBLESHOOTING

### Common Issues & Solutions

```
• 'No PRs found' - Check your .env configuration and filters
• Notification errors - Ensure macOS notifications are enabled for your terminal
• Claude errors - Verify claude CLI is installed and logged in
• Monitor won't start - Check if another instance is running (monitor-status)
• Instructions not loading - Verify instruction.md file exists and is readable
• 'git init' requests - Safe to decline, tool doesn't need git repo access
```

### macOS Notifications Not Working

If you're not receiving macOS notifications:

1. **Check Terminal Permissions**:
   ```bash
   open "x-apple.systempreferences:com.apple.preference.security?Privacy_Notifications"
   ```
   Ensure your terminal app has notification permissions enabled.

2. **Check System Settings**:
   ```bash
   open -a "System Settings" --args notifications
   ```
   - Disable "Do Not Disturb"
   - Check "Focus" mode isn't active
   - Ensure notifications aren't silenced

3. **Test Notifications**:
   ```bash
   osascript -e 'display notification "Test" with title "🦀 Review Dispatcher" subtitle "Test Notification" sound name "Glass"'
   ```
   If this doesn't work, the issue is with your macOS setup.

4. **Terminal-Specific Issues**:
   - **iTerm2**: Check iTerm2 → Settings → Profiles → [Your Profile] → Terminal → Notifications
   - **Warp**: Ensure app has notification permissions
   - **Default Terminal**: Check System Settings → Notifications → Terminal

5. **Chrome Auto-Open Issues**:
   If notifications appear but Chrome doesn't open:
   - Verify Chrome is installed at `/Applications/Google Chrome.app`
   - Test Chrome opening: `osascript -e 'tell application "Google Chrome" to activate'`
   - Check Chrome permissions in System Settings → Privacy & Security

6. **Notification Content Issues**:
   If notifications show but content is wrong:
   - Check the PR URL format in your configuration
   - Verify the notification format matches expected patterns
   - Test with a simple notification first

### Instruction File Not Loading

The program searches for instructions in this order:
1. `--instruction-path` flag
2. `RD_INSTRUCTION_PATH` environment variable
3. `./instruction.md` (current directory)
4. `~/.review-dispatcher/instruction.md` (user config)
5. Built-in defaults

**Debug instruction loading:**
```bash
# Check all possible locations
echo "Current dir: $(pwd)/instruction.md"
echo "Config dir: ~/.review-dispatcher/instruction.md"
echo "Env var: $RD_INSTRUCTION_PATH"

# Test with explicit path
review-dispatcher delegate --instruction-path /full/path/to/instructions.md
```

### Monitor Not Detecting New PRs

1. **Check GitHub Token Permissions**:
   ```bash
   # Test token access
   TOKEN=$(grep RD_GITHUB_TOKEN .env | cut -d'=' -f2)
   curl -H "Authorization: token $TOKEN" https://api.github.com/user
   
   # Test repository access
   curl -H "Authorization: token $TOKEN" \
        https://api.github.com/repos/your-org/your-repo
   ```
   
   **Common Issues**:
   - Token doesn't have `repo` scope
   - Token not granted access to specific repository
   - Organization membership required
   - Repository name/org name typo in .env
   
   **Solution**: Edit token to add repository access or request access from org admin.

2. **Verify Repository Configuration**:
   ```bash
   # Check your .env file
   cat .env | grep RD_GITHUB
   ```

3. **Test with Smaller Interval**:
   ```bash
   review-dispatcher monitor -i 10  # 10 second intervals
   ```

4. **Check for API Errors**:
   ```bash
   # Run with debug logging
   RUST_LOG=debug review-dispatcher monitor
   ```

### Path Logging Not Showing

The program shows instruction paths in these scenarios:

**Monitor Mode:**
```
review-dispatcher monitor
👀 Starting PR monitor...
📖 Monitor will use instructions from: /path/to/instructions.md
```

**Delegate Mode:**
```
review-dispatcher delegate 123
📖 Using custom instructions from: /path/to/instructions.md
⏳ Delegating PR #123 to Claude...
```

If you don't see these messages:
- Check if the instruction file exists at expected locations
- Verify file permissions (`chmod 644 instruction.md`)
- Test with explicit path: `--instruction-path /full/path/to/file.md`

### Debugging Tips

**Enable Debug Logging:**
```bash
RUST_BACKTRACE=1 review-dispatcher monitor
```

**Check System Logs:**
```bash
log stream --predicate 'process == "review-dispatcher"'
```

**Verify Configuration:**
```bash
# Check all environment variables
env | grep RD_

# Test GitHub API access
curl -H "Authorization: token $(grep RD_GITHUB_TOKEN .env | cut -d'=' -f2)" \
     https://api.github.com/user
```

## License

MIT
