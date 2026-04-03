use anyhow::{Context, Result};
use std::process::{Command, Stdio};

const PRCTRL_DOCS: &str = r#"# PRCtrl CLI Documentation

## Overview
PRCtrl is a GitHub PR review management CLI tool.

## Commands
- list, delegate, stats, team, workload, clean, monitor, stop, watch
- diff, info, timeline, assign, unassign, comment, approve, claim
- open, files, report, search, filter, ci, conflicts, labels
- history, notify, summary, urgent, focus, health, top, quick
- catchup, age, size, cat, digest, trends, velocity
- snooze, chase, estimate, export, history, ready, compare
- follow, blocked, config

## Configuration
Config file: ~/.prctrl/config.toml

## Examples
prctrl list
prctrl list --priority --repo backend
prctrl delegate --all
prctrl approve --pr 123
prctrl chat --pr 123
"#;

/// Check if the Claude CLI is installed
pub fn get_backend() -> Option<&'static str> {
    if Command::new("claude")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
    {
        Some("claude")
    } else {
        None
    }
}

pub fn start_chat(_backend: &str, pr_number: Option<u64>) -> Result<()> {
    // Check if Claude CLI exists first
    if Command::new("claude")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        anyhow::bail!("Claude CLI not found. Install: https://docs.anthropic.com/claude-cli");
    }

    let system_prompt = if let Some(pr) = pr_number {
        format!(r#"You are a helpful PR review assistant. The user is asking about PR #{}.

{}

When suggesting commands, use format: `prctrl <command>`
"#, pr, PRCTRL_DOCS)
    } else {
        format!(r#"You are a helpful PR review assistant.

{}

When suggesting commands, use format: `prctrl <command>`
"#, PRCTRL_DOCS)
    };

    println!("\n🤖 PRCtrl Chat - Starting Claude...\n");
    println!("Type your questions, or exit to quit.\n\n");

    // Launch Claude Code interactively
    let status = Command::new("claude")
        .arg("--model")
        .arg("sonnet")
        .arg("--append-system-prompt")
        .arg(&system_prompt)
        .status()
        .context("Failed to run Claude")?;
    
    if !status.success() {
        eprintln!("\n⚠️  Claude exited with status: {}", status);
    }

    Ok(())
}
