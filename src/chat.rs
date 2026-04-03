use anyhow::Result;
use std::io::{BufRead, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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

fn animated_loader(stop_flag: Arc<AtomicBool>) {
    let spinner = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let mut i = 0;
    while !stop_flag.load(Ordering::Relaxed) {
        print!("\r🤖 Claude is thinking... {}", spinner[i % spinner.len()]);
        std::io::stdout().flush().ok();
        thread::sleep(Duration::from_millis(80));
        i += 1;
    }
    print!("\r");
    std::io::stdout().flush().ok();
}

pub fn start_chat(_backend: &str, pr_number: Option<u64>) -> Result<()> {
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
    println!("Type 'exit' to quit.\n");

    let mut child = Command::new("claude")
        .args(["--print", "--model", "sonnet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn claude");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    let stdout = child.stdout.as_mut().expect("Failed to capture stdout");

    // Send system prompt
    stdin.write_all(system_prompt.as_bytes())?;
    stdin.write_all(b"\n\nHi! I'm ready to help with PRCtrl. Ask me anything about your PRs.\n\n")?;
    stdin.flush()?;

    // Shared flag to control loader
    let loading = Arc::new(AtomicBool::new(false));

    // Main loop - read user input and send to Claude
    loop {
        print!("you: ");
        std::io::stdout().flush()?;
        
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            break;
        }
        let input = input.trim();
        
        if input.to_lowercase() == "exit" || input.to_lowercase() == "quit" {
            println!("\n👋 Goodbye!\n");
            break;
        }

        // Send input and start loader
        stdin.write_all(input.as_bytes())?;
        stdin.write_all(b"\n")?;
        stdin.flush()?;

        // Start animated loader
        loading.store(true, Ordering::Relaxed);
        let stop_loader = Arc::new(AtomicBool::new(false));
        let stop_clone = stop_loader.clone();
        
        let loader_handle = thread::spawn(move || {
            animated_loader(stop_clone);
        });

        // Read all response lines
        let reader = std::io::BufReader::new(&mut *stdout);
        let mut response_lines = Vec::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                // Check if this looks like a new prompt (user input echo)
                if line.starts_with("you:") || line.is_empty() {
                    break;
                }
                response_lines.push(line);
            }
        }

        // Stop loader and show response
        stop_loader.store(true, Ordering::Relaxed);
        let _ = loader_handle.join();
        
        if !response_lines.is_empty() {
            println!();
            for line in &response_lines {
                println!("{}", line);
            }
            println!();
        }
    }

    Ok(())
}
