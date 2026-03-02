mod cli;
mod config;
mod dispatcher;
mod github;
mod logger;
mod notifications;
mod terminal;
mod writer;

use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = config::Config::from_env()?;
    let cli = Cli::parse();

    let include_mine = cli.include_mine || cli.crew;
    let include_drafts = cli.include_drafts || cli.crew;
    let crew_members = if cli.crew { &cfg.crew_members } else { &vec![] };

    let reviews = github::fetch_pending_reviews(
        &cfg.github_token,
        &cfg.github_org,
        &cfg.github_repos,
        &cfg.github_username,
        &cfg.github_teams,
        include_mine,
        include_drafts,
        &cli.exclude_prefix,
        crew_members,
    )
    .await?;

    // Resolve output dir (default: ./reviews/)
    let output_dir: Option<PathBuf> = cli.output_dir.clone().or_else(|| Some(PathBuf::from("./reviews")));

    match cli.command {
        Commands::List => {
            // --pr on list: filter the review list to that PR
            let filtered: Vec<_> = match cli.pr {
                Some(num) => reviews.iter().filter(|r| r.pr_number == num).cloned().collect(),
                None => reviews.clone(),
            };

            logger::print_reviews(&filtered);

            if let Some(ref dir) = output_dir {
                let index_path = writer::write_index(dir, &filtered)?;
                for review in &filtered {
                    writer::write_review(dir, review, None)?;
                }
                println!(
                    "\n📁 Written to {}  (index: {})",
                    dir.display().to_string().cyan(),
                    index_path.file_name().unwrap().to_string_lossy().dimmed()
                );
            }
        }

        Commands::Mine => {
            let my_prs = github::fetch_my_open_prs(
                &cfg.github_token,
                &cfg.github_org,
                &cfg.github_repos,
                &cfg.github_username,
                cli.include_drafts,
                &cli.exclude_prefix,
            )
            .await?;

            logger::print_reviews(&my_prs);

            if let Some(ref dir) = output_dir {
                let index_path = writer::write_index(dir, &my_prs)?;
                for review in &my_prs {
                    writer::write_review(dir, review, None)?;
                }
                println!(
                    "\n📁 Written to {}  (index: {})",
                    dir.display().to_string().cyan(),
                    index_path.file_name().unwrap().to_string_lossy().dimmed()
                );
            }
        }

        Commands::Delegate { pr_positional } => {
            // --pr flag takes precedence, then positional arg
            let pr_number = cli.pr.or(pr_positional);

            let targets: Vec<_> = match pr_number {
                Some(num) => {
                    // Fetch the PR directly, bypassing review-request filters
                    let direct = github::fetch_pr_by_number(
                        &cfg.github_token,
                        &cfg.github_org,
                        &cfg.github_repos,
                        num,
                    )
                    .await?;
                    if direct.is_empty() {
                        println!("No PR #{} found across configured repos.", num);
                        return Ok(());
                    }
                    if direct.len() == 1 {
                        direct
                    } else {
                        // Same PR number in multiple repos — show matches + prompt
                        println!("PR #{} found in multiple repos:", num);
                        for (i, r) in direct.iter().enumerate() {
                            println!("  [{}] {} ({})", i + 1, r.pr_title, r.repo);
                        }
                        print!(
                            "\n{} ",
                            "Select repo [e.g. 1] (q to quit):".bold()
                        );
                        io::stdout().flush()?;
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        match parse_selection(input.trim(), direct.len()) {
                            Selection::Quit => return Ok(()),
                            Selection::Indices(indices) => {
                                indices.into_iter().map(|i| direct[i].clone()).collect()
                            }
                        }
                    }
                }
                None => {
                    if reviews.is_empty() {
                        println!("No matching reviews found.");
                        return Ok(());
                    }

                    logger::print_reviews(&reviews);

                    print!(
                        "\n{} ",
                        "Select PRs to delegate [e.g. 1,3 or 1-3 or 'all'] (q to quit):".bold()
                    );
                    io::stdout().flush()?;

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let input = input.trim();

                    match parse_selection(input, reviews.len()) {
                        Selection::Quit => return Ok(()),
                        Selection::Indices(indices) => {
                            indices.into_iter().map(|i| reviews[i].clone()).collect()
                        }
                    }
                }
            };

            if targets.is_empty() {
                println!("No matching reviews found.");
                return Ok(());
            }

            for (i, review) in targets.iter().enumerate() {
                print!(
                    "\n⏳ Delegating [{}/{}] #{} {}... ",
                    i + 1,
                    targets.len(),
                    review.pr_number,
                    review.pr_title
                );
                io::stdout().flush()?;
                let summary = dispatcher::delegate_to_claude(review, cli.instruction_path.clone())?;
                println!("{}", "done".green());
                logger::print_delegate_result(review, &summary);

                if let Some(ref dir) = output_dir {
                    let path = writer::write_review(dir, review, Some(&summary))?;
                    println!("   💾 Saved → {}", path.display().to_string().dimmed());
                }
            }

            if let Some(ref dir) = output_dir {
                writer::write_index(dir, &reviews)?;
            }
        }

        Commands::Clean => {
            if let Some(ref dir) = output_dir {
                if dir.exists() {
                    let count = std::fs::read_dir(dir)?
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().is_file())
                        .count();
                    std::fs::remove_dir_all(dir)?;
                    println!(
                        "🧹 Removed {} file(s) from {}",
                        count,
                        dir.display().to_string().cyan()
                    );
                } else {
                    println!("Nothing to clean — {} does not exist.", dir.display());
                }
            }
        }
        
        Commands::Monitor { interval, notify, auto_open, interactive } => {
            println!("👀 Starting PR monitor (polling every {} seconds)...", interval);
            if interactive {
                println!("🎮 Interactive mode enabled - will prompt for actions on new PRs");
            }
            if notify {
                if auto_open {
                    println!("🔔 Notifications enabled with auto-open in Chrome");
                } else {
                    println!("🔔 Notifications enabled (URLs shown in message)");
                }
            } else {
                println!("🔔 Notifications disabled");
            }
            println!("Press Ctrl+C to stop.");
            
            dispatcher::monitor_new_prs(
                &cfg.github_token,
                &cfg.github_org,
                &cfg.github_repos,
                &cfg.github_username,
                &cfg.github_teams,
                include_mine,
                include_drafts,
                &cli.exclude_prefix,
                crew_members,
                interval,
                notify,
                auto_open,
                interactive,
                output_dir.clone(),
            )
            .await?;
        }
        
        Commands::MonitorStop => {
            println!("🛑 Stopping monitor process...");
            match dispatcher::kill_existing_monitor() {
                Ok(true) => println!("✓ Monitor process stopped successfully"),
                Ok(false) => println!("⚠ No running monitor process found"),
                Err(e) => println!("❌ Error stopping monitor: {}", e),
            }
        }
        
        Commands::MonitorStatus => {
            if dispatcher::is_monitor_running() {
                println!("✅ Monitor process is running");
                if let Ok(pid_str) = fs::read_to_string(dispatcher::PID_FILE) {
                    println!("   PID: {}", pid_str.trim());
                }
            } else {
                println!("❌ Monitor process is not running");
            }
        }
    }

    // Open terminal tab last, after all files are written
    if cli.open_terminal {
        if let Some(ref dir) = output_dir {
            std::fs::create_dir_all(dir)?;
            terminal::open_terminal_at(dir)?;
            println!(
                "🖥️  Opened terminal at {}",
                dir.display().to_string().cyan()
            );
        }
    }

    Ok(())
}

enum Selection {
    Quit,
    Indices(Vec<usize>),
}

fn parse_selection(input: &str, total: usize) -> Selection {
    let input = input.trim().to_lowercase();

    if input == "q" || input == "quit" {
        return Selection::Quit;
    }

    if input == "all" {
        return Selection::Indices((0..total).collect());
    }

    let mut indices = BTreeSet::new();

    for part in input.split(',') {
        let part = part.trim();
        if let Some((start, end)) = part.split_once('-') {
            if let (Ok(s), Ok(e)) = (start.trim().parse::<usize>(), end.trim().parse::<usize>()) {
                if s >= 1 && e >= s && e <= total {
                    for i in s..=e {
                        indices.insert(i - 1);
                    }
                }
            }
        } else if let Ok(n) = part.parse::<usize>() {
            if n >= 1 && n <= total {
                indices.insert(n - 1);
            }
        }
    }

    Selection::Indices(indices.into_iter().collect())
}
