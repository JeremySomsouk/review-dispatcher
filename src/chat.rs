use anyhow::Result;
use colored::Colorize;
use std::io::Write;
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

pub fn get_backend() -> Option<&'static str> {
    if Command::new("claude")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
    {
        return Some("claude");
    }
    None
}

pub fn start_chat(backend: &str, pr_number: Option<u64>) -> Result<()> {
    let system_prompt = if let Some(pr) = pr_number {
        format!(r#"You are a helpful PR review assistant. The user is asking about PR #{}.

PRCtrl commands: list, delegate, stats, team, diff, info, timeline, assign, unassign, comment, approve, claim, open, files, search, filter, ci, conflicts, labels, history, notify, summary, urgent, focus, health, top, quick, catchup, age, size, cat, digest, trends, velocity, snooze, chase, estimate, export, ready, compare, follow, blocked, config

When suggesting commands, use format: `prctrl <command>`
"#, pr)
    } else {
        r#"You are a helpful PR review assistant.

PRCtrl commands: list, delegate, stats, team, diff, info, timeline, assign, unassign, comment, approve, claim, open, files, search, filter, ci, conflicts, labels, history, notify, summary, urgent, focus, health, top, quick, catchup, age, size, cat, digest, trends, velocity, snooze, chase, estimate, export, ready, compare, follow, blocked, config

When suggesting commands, use format: `prctrl <command>`
"#.to_string()
    };

    println!("\n🤖 PRCtrl Chat - Starting {}...\n", backend.cyan());
    println!("Type 'exit' to quit.\n");

    let mut child = Command::new(backend)
        .args(["--print", "--model", "sonnet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn chat backend");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    
    stdin.write_all(system_prompt.as_bytes())?;
    stdin.write_all(b"\n\nHi! I'm ready to help with PRCtrl. What would you like to do?\n\n")?;

    print!("{} ", "you:".blue().bold());
    std::io::stdout().flush()?;
    
    loop {
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            break;
        }
        let input = input.trim();
        
        if input.to_lowercase() == "exit" || input.to_lowercase() == "quit" {
            println!("\n👋 Goodbye!\n");
            break;
        }
        
        stdin.write_all(input.as_bytes())?;
        stdin.write_all(b"\n")?;
        
        print!("{} ", "you:".blue().bold());
        std::io::stdout().flush()?;
    }

    Ok(())
}
