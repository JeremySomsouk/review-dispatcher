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
use open;
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
        Commands::List { json, since_days, priority } => {
            // --pr on list: filter the review list to that PR
            let filtered: Vec<_> = match cli.pr {
                Some(num) => reviews.iter().filter(|r| r.pr_number == num).cloned().collect(),
                None => reviews.clone(),
            };

            // Apply --since filter
            let filtered: Vec<_> = match since_days {
                Some(days) => {
                    let cutoff = chrono::Utc::now() - chrono::Duration::days(days as i64);
                    filtered
                        .into_iter()
                        .filter(|r| r.created_at >= cutoff)
                        .collect()
                }
                None => filtered,
            };

            if json {
                let json = serde_json::to_string_pretty(&filtered)?;
                println!("{}", json);
            } else {
                logger::print_reviews(&filtered, priority);
            }

            // Write files only in non-JSON mode (JSON is typically for scripting)
            if !json {
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

            logger::print_reviews(&my_prs, false);

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

                    logger::print_reviews(&reviews, false);

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

        Commands::Stats => {
            use std::collections::HashMap;
            use chrono::Duration;

            let mut repo_counts: HashMap<String, usize> = HashMap::new();
            let mut total_additions = 0u64;
            let mut total_deletions = 0u64;

            for review in &reviews {
                *repo_counts.entry(review.repo.clone()).or_insert(0) += 1;
                total_additions += review.additions;
                total_deletions += review.deletions;
            }

            println!("\n📊 Review Statistics\n{}", "─".repeat(40));
            println!("  Total pending reviews: {}", reviews.len());
            println!("  Total lines changed:   +{} / -{}", total_additions, total_deletions);

            if !reviews.is_empty() {
                // Average wait time
                let now = chrono::Utc::now();
                let total_wait: Duration = reviews.iter().map(|r| now - r.created_at).sum();
                let avg_wait = total_wait / reviews.len() as i32;
                println!("  Avg time waiting:      {} days",
                    (avg_wait.num_hours() as f64 / 24.0).round());

                // Oldest PR
                if let Some(oldest) = reviews.first() {
                    let age = now - oldest.created_at;
                    println!("  Oldest PR:             #{} ({} ago)", oldest.pr_number,
                        format_duration(age));
                }

                // Breakdown by repo
                println!("\n  By repository:");
                let mut repo_vec: Vec<_> = repo_counts.iter().collect();
                repo_vec.sort_by(|a, b| b.1.cmp(a.1));
                for (repo, count) in repo_vec {
                    println!("    {}: {}", repo, count);
                }
            }

            println!();
        }

        Commands::TeamSummary => {
            use std::collections::HashMap;

            let mut team_counts: HashMap<String, usize> = HashMap::new();
            let mut unassigned = 0usize;

            for review in &reviews {
                if review.pr_author.is_empty() {
                    unassigned += 1;
                } else {
                    *team_counts.entry(review.pr_author.clone()).or_insert(0) += 1;
                }
            }

            println!("\n👥 Team Review Summary\n{}", "─".repeat(40));
            println!("  Total pending reviews: {}", reviews.len());

            if !team_counts.is_empty() {
                println!("\n  By author:");
                let mut sorted: Vec<_> = team_counts.iter().collect();
                sorted.sort_by(|a, b| b.1.cmp(a.1)); // descending by count
                for (author, count) in sorted {
                    let bar = "█".repeat(*count).cyan();
                    println!("    {}  {}", author.bold(), bar);
                }
            }

            if unassigned > 0 {
                println!("\n  Unassigned/Unknown: {}", unassigned);
            }

            // Show breakdown by repository
            let mut repo_counts: HashMap<String, usize> = HashMap::new();
            for review in &reviews {
                *repo_counts.entry(review.repo.clone()).or_insert(0) += 1;
            }
            println!("\n  By repository:");
            let mut repo_sorted: Vec<_> = repo_counts.iter().collect();
            repo_sorted.sort_by(|a, b| b.1.cmp(a.1));
            for (repo, count) in repo_sorted {
                println!("    {}: {}", repo, count);
            }

            println!();
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
        
        Commands::Monitor { interval, notify, auto_open, no_auto_open, interactive } => {
            let effective_auto_open = auto_open && !no_auto_open;
            
            println!("👀 Starting PR monitor (polling every {} seconds)...", interval);
            if interactive {
                println!("🎮 Interactive mode enabled - will prompt for actions on new PRs");
            }
            if notify {
                if effective_auto_open {
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
                effective_auto_open,
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

        Commands::Diff { pr_number } => {
            let target_pr = cli.pr.or(pr_number);

            let prs = match target_pr {
                Some(num) => {
                    github::fetch_pr_by_number(
                        &cfg.github_token,
                        &cfg.github_org,
                        &cfg.github_repos,
                        num,
                    )
                    .await?
                }
                None => {
                    // Show all pending reviews and let user pick
                    if reviews.is_empty() {
                        println!("No pending reviews found.");
                        return Ok(());
                    }
                    logger::print_reviews(&reviews, false);
                    print!(
                        "\n{} ",
                        "Select PR to diff [e.g. 1 or 1,3 or 1-3] (q to quit):".bold()
                    );
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    match parse_selection(input.trim(), reviews.len()) {
                        Selection::Quit => return Ok(()),
                        Selection::Indices(indices) => {
                            indices.into_iter().map(|i| reviews[i].clone()).collect()
                        }
                    }
                }
            };

            if prs.is_empty() {
                println!("No PR found to diff.");
                return Ok(());
            }

            for review in prs {
                let age_days = (chrono::Utc::now() - review.created_at).num_days();
                let age_label = match age_days {
                    0 => "today".green(),
                    1 => "1 day ago".normal(),
                    _ => format!("{} days ago", age_days).red(),
                };

                println!("\n{}", "─".repeat(60));
                println!("📄 {}  #{}", review.pr_title.bold(), review.pr_number);
                println!("   👤 {}  •  📅 {}  •  🌿 {}", 
                    review.pr_author.cyan(), 
                    age_label,
                    review.branch.dimmed()
                );
                println!("   📊 {}  •  +{} additions  •  -{} deletions",
                    if review.draft { "DRAFT".yellow() } else { "READY".green() },
                    review.additions.to_string().green(),
                    review.deletions.to_string().red()
                );
                println!("   🔗 {}", review.pr_url.blue().underline());
                println!("{}", "─".repeat(60));

                // Show size category
                let total = review.additions + review.deletions;
                let size_label = if total < 50 {
                    "XS".to_string()
                } else if total < 200 {
                    "S".to_string()
                } else if total < 500 {
                    "M".to_string()
                } else if total < 1000 {
                    "L".to_string()
                } else {
                    "XL".to_string()
                };
                let size_color: colored::Color = match size_label.as_str() {
                    "XS" | "S" => colored::Color::Green,
                    "M" => colored::Color::Yellow,
                    "L" => colored::Color::Red,
                    _ => colored::Color::Magenta,
                };
                println!("   📦 Size: {} ({} lines)", size_label.color(size_color), total);

                // Show age category
                let age_cat = if age_days == 0 {
                    "🔥 HOT"
                } else if age_days <= 2 {
                    "⚡ FRESH"
                } else if age_days <= 7 {
                    "📅 WEEK OLD"
                } else if age_days <= 14 {
                    "⚠️  STALE"
                } else {
                    "🚨 OLD"
                };
                println!("   ⏱️  Age: {} ({} days)", age_cat, age_days);

                // Priority indicator
                let score = logger::calculate_priority_score(&review);
                println!("   ⭐ Priority: {}/5  {}", score, logger::priority_stars(score));

                // Show repo
                println!("   📁 Repository: {}", review.repo);
                println!("{}", "─".repeat(60));
                println!();
            }
        }

        Commands::Assign { pr_number } => {
            let target_pr = cli.pr.or(pr_number);

            let prs = match target_pr {
                Some(num) => {
                    github::fetch_pr_by_number(
                        &cfg.github_token,
                        &cfg.github_org,
                        &cfg.github_repos,
                        num,
                    )
                    .await?
                }
                None => {
                    if reviews.is_empty() {
                        println!("No pending reviews found.");
                        return Ok(());
                    }
                    logger::print_reviews(&reviews, false);
                    print!(
                        "\n{} ",
                        "Select PR to assign yourself [e.g. 1 or 1,3 or 1-3] (q to quit):".bold()
                    );
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    match parse_selection(input.trim(), reviews.len()) {
                        Selection::Quit => return Ok(()),
                        Selection::Indices(indices) => {
                            indices.into_iter().map(|i| reviews[i].clone()).collect()
                        }
                    }
                }
            };

            if prs.is_empty() {
                println!("No PR found to assign.");
                return Ok(());
            }

            for review in prs {
                print!(
                    "\n⏳ Requesting review on #{} {}... ",
                    review.pr_number,
                    review.pr_title
                );
                io::stdout().flush()?;

                let client = octocrab::Octocrab::builder()
                    .personal_token(cfg.github_token.clone())
                    .build()?;

                let result = client
                    .pulls(&cfg.github_org, &review.repo)
                    .request_reviews(
                        review.pr_number,
                        vec![cfg.github_username.clone()],
                        Vec::<String>::new(),
                    )
                    .await;

                match result {
                    Ok(_) => {
                        println!("{}", "✅ Assigned".green());
                        println!("   👤 You're now a reviewer on {} ({})", review.pr_title, review.repo);
                        println!("   🔗 {}", review.pr_url.blue().underline());
                    }
                    Err(e) => {
                        println!("{}", "❌ Failed".red());
                        println!("   Error: {}", e);
                    }
                }
            }
            println!();
        }

        Commands::Browse { pr_numbers } => {
            let targets: Vec<_> = if let Some(ref nums) = pr_numbers {
                // Parse comma-separated PR numbers
                let mut results = Vec::new();
                for part in nums.split(',') {
                    if let Ok(num) = part.trim().parse::<u64>() {
                        results.push(num);
                    }
                }
                if results.is_empty() {
                    println!("❌ No valid PR numbers provided.");
                    return Ok(());
                }
                // Fetch all specified PRs
                let mut all_prs = Vec::new();
                for num in &results {
                    let prs = github::fetch_pr_by_number(
                        &cfg.github_token,
                        &cfg.github_org,
                        &cfg.github_repos,
                        *num,
                    )
                    .await?;
                    all_prs.extend(prs);
                }
                all_prs
            } else {
                // Interactive: show list and let user pick
                if reviews.is_empty() {
                    println!("No pending reviews found.");
                    return Ok(());
                }
                logger::print_reviews(&reviews, false);
                print!(
                    "\n{} ",
                    "Select PRs to open [e.g. 1,3 or 1-3 or 'all'] (q to quit):".bold()
                );
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                match parse_selection(input.trim(), reviews.len()) {
                    Selection::Quit => return Ok(()),
                    Selection::Indices(indices) => {
                        indices.into_iter().map(|i| reviews[i].clone()).collect()
                    }
                }
            };

            if targets.is_empty() {
                println!("No PRs found to open.");
                return Ok(());
            }

            println!("\n🌐 Opening {} PR(s) in browser...\n", targets.len());
            for review in &targets {
                match open::that(&review.pr_url) {
                    Ok(_) => {
                        println!("  ✅ {} ({})", review.pr_title.dimmed(), review.pr_url.cyan());
                    }
                    Err(e) => {
                        println!("  ❌ Failed to open {}: {}", review.pr_url, e);
                    }
                }
            }
            println!();
        }

        Commands::Files { pr_numbers, all } => {
            let targets: Vec<_> = if all {
                // Show files for all pending reviews
                if reviews.is_empty() {
                    println!("No pending reviews found.");
                    return Ok(());
                }
                reviews.clone()
            } else if let Some(ref nums) = pr_numbers {
                // Parse comma-separated PR numbers
                let mut results = Vec::new();
                for part in nums.split(',') {
                    if let Ok(num) = part.trim().parse::<u64>() {
                        results.push(num);
                    }
                }
                if results.is_empty() {
                    println!("❌ No valid PR numbers provided.");
                    return Ok(());
                }
                // Fetch all specified PRs
                let mut all_prs = Vec::new();
                for num in &results {
                    let prs = github::fetch_pr_by_number(
                        &cfg.github_token,
                        &cfg.github_org,
                        &cfg.github_repos,
                        *num,
                    )
                    .await?;
                    all_prs.extend(prs);
                }
                all_prs
            } else {
                // Interactive: show list and let user pick
                if reviews.is_empty() {
                    println!("No pending reviews found.");
                    return Ok(());
                }
                logger::print_reviews(&reviews, false);
                print!(
                    "\n{} ",
                    "Select PRs to show files [e.g. 1,3 or 1-3 or 'all'] (q to quit):".bold()
                );
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                match parse_selection(input.trim(), reviews.len()) {
                    Selection::Quit => return Ok(()),
                    Selection::Indices(indices) => {
                        indices.into_iter().map(|i| reviews[i].clone()).collect()
                    }
                }
            };

            if targets.is_empty() {
                println!("No PRs found.");
                return Ok(());
            }

            // Show files for each selected PR
            for review in &targets {
                print!("\n⏳ Fetching files for #{} {}... ", review.pr_number, review.pr_title);
                io::stdout().flush()?;

                match github::fetch_pr_files(
                    &cfg.github_token,
                    &cfg.github_org,
                    &review.repo,
                    review.pr_number,
                )
                .await
                {
                    Ok(files) => {
                        println!("{}", "done".green());
                        println!("\n{}", "─".repeat(60));
                        println!("📄 {}  #{}  ({} files)", review.pr_title.bold(), review.pr_number, files.len());
                        println!("{}", "─".repeat(60));

                        if files.is_empty() {
                            println!("  (no file changes)");
                        } else {
                            for file in &files {
                                let status_icon = match file.status.as_str() {
                                    "added" => "+".green(),
                                    "removed" => "-".red(),
                                    "modified" => "M".yellow(),
                                    "renamed" => "R".cyan(),
                                    _ => "?".normal(),
                                };
                                let total = file.additions + file.deletions;
                                let size_indicator = if total > 500 {
                                    format!(" ({} lines)", total).red()
                                } else if total > 200 {
                                    format!(" ({} lines)", total).yellow()
                                } else {
                                    format!(" ({} lines)", total).dimmed()
                                };
                                println!(
                                    "  {}  {}{}",
                                    status_icon,
                                    file.filename.dimmed(),
                                    size_indicator
                                );
                            }
                        }
                        println!("{}", "─".repeat(60));
                    }
                    Err(e) => {
                        println!("{}", "failed".red());
                        println!("  ❌ Error fetching files: {}", e);
                    }
                }
            }
            println!();
        }

        Commands::Filter { repo, author, min_size, max_size, min_age, max_age, drafts_only, no_drafts, priority, json } => {
            // Apply filters to the reviews
            let filtered: Vec<_> = reviews.iter().filter(|r| {
                // Filter by repo (partial match, case-insensitive)
                if let Some(ref repo_filter) = repo {
                    if !r.repo.to_lowercase().contains(&repo_filter.to_lowercase()) {
                        return false;
                    }
                }

                // Filter by author (partial match, case-insensitive)
                if let Some(ref author_filter) = author {
                    if !r.pr_author.to_lowercase().contains(&author_filter.to_lowercase()) {
                        return false;
                    }
                }

                // Filter by size
                let total_size = r.additions + r.deletions;
                if let Some(min) = min_size {
                    if total_size < min {
                        return false;
                    }
                }
                if let Some(max) = max_size {
                    if total_size > max {
                        return false;
                    }
                }

                // Filter by age
                let age_days = (chrono::Utc::now() - r.created_at).num_days() as u32;
                if let Some(min) = min_age {
                    if age_days < min {
                        return false;
                    }
                }
                if let Some(max) = max_age {
                    if age_days > max {
                        return false;
                    }
                }

                // Filter by draft status
                if drafts_only && !r.draft {
                    return false;
                }
                if no_drafts && r.draft {
                    return false;
                }

                true
            }).cloned().collect();

            if filtered.is_empty() {
                println!("\n🔍 No reviews match the specified filters.\n");
                return Ok(());
            }

            if json {
                let json_output = serde_json::to_string_pretty(&filtered)?;
                println!("{}", json_output);
            } else {
                println!(
                    "\n🔍 {} review(s) match your filters\n",
                    filtered.len().to_string().yellow().bold()
                );
                logger::print_reviews(&filtered, priority);
            }
        }

        Commands::Report { days, json } => {
            use chrono::{Duration, Utc};
            use std::collections::HashMap;

            let report_output_dir = output_dir.clone().unwrap_or_else(|| PathBuf::from("./reviews"));

            if !report_output_dir.exists() {
                println!("❌ No reviews directory found at {}. Run `review-dispatcher list` first to save reviews.", report_output_dir.display());
                return Ok(());
            }

            // Read all review files from the output directory
            let mut processed_count = 0u32;
            let mut processed_by_author: HashMap<String, u32> = HashMap::new();
            let mut processed_by_repo: HashMap<String, u32> = HashMap::new();
            let mut recent_reviews: Vec<(String, chrono::DateTime<Utc>, String)> = vec![];

            let cutoff = Utc::now() - Duration::days(days as i64);

            if let Ok(entries) = std::fs::read_dir(&report_output_dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("md") {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            let lines: Vec<&str> = content.lines().collect();
                            if lines.len() >= 4 {
                                let pr_title = lines.first().unwrap_or(&"").trim();
                                let date_line = lines.iter().find(|l| l.starts_with("Reviewed on"));
                                if let Some(date_str) = date_line {
                                    if let Some(date_part) = date_str.strip_prefix("Reviewed on ") {
                                        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_part) {
                                            let date = date.with_timezone(&Utc);
                                            if date >= cutoff {
                                                processed_count += 1;
                                                recent_reviews.push((pr_title.to_string(), date, path.file_name().unwrap_or_default().to_string_lossy().to_string()));

                                                for line in &lines {
                                                    if line.starts_with("- **Author**:") {
                                                        if let Some(author) = line.strip_prefix("- **Author**:") {
                                                            let author = author.trim();
                                                            *processed_by_author.entry(author.to_string()).or_insert(0) += 1;
                                                        }
                                                    }
                                                    if line.starts_with("- **Repository**:") {
                                                        if let Some(repo) = line.strip_prefix("- **Repository**:") {
                                                            let repo = repo.trim();
                                                            *processed_by_repo.entry(repo.to_string()).or_insert(0) += 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let pending_total = reviews.len();
            let pending_old = reviews.iter().filter(|r| {
                let age_days = (Utc::now() - r.created_at).num_days() as i64;
                age_days >= days as i64
            }).count();

            let pending_additions: u64 = reviews.iter().map(|r| r.additions).sum();
            let pending_deletions: u64 = reviews.iter().map(|r| r.deletions).sum();

            let report = serde_json::json!({
                "period_days": days,
                "generated_at": Utc::now().to_rfc3339(),
                "processed_reviews": processed_count,
                "pending_reviews": pending_total,
                "pending_old_count": pending_old,
                "pending_additions": pending_additions,
                "pending_deletions": pending_deletions,
                "by_author": processed_by_author,
                "by_repository": processed_by_repo
            });

            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("\n📊 Weekly Review Report  ({}-day period)\n{}", days, "─".repeat(45));
                println!("  📁 Directory: {}", report_output_dir.display().to_string().dimmed());
                println!();
                println!("  ✅ Processed Reviews:");
                println!("     Total reviewed:     {}", processed_count);
                if !processed_by_author.is_empty() {
                    println!("     By author:");
                    let mut sorted: Vec<_> = processed_by_author.iter().collect();
                    sorted.sort_by(|a, b| b.1.cmp(a.1));
                    for (author, count) in sorted.iter().take(5) {
                        println!("       {}: {}", author.cyan(), count);
                    }
                }
                if !processed_by_repo.is_empty() {
                    println!("     By repository:");
                    let mut sorted: Vec<_> = processed_by_repo.iter().collect();
                    sorted.sort_by(|a, b| b.1.cmp(a.1));
                    for (repo, count) in sorted.iter().take(5) {
                        println!("       {}: {}", repo, count);
                    }
                }
                println!();
                println!("  ⏳ Current Pending:");
                println!("     Total pending:       {}", pending_total);
                println!("     Old ({}d+):          {}", days, pending_old);
                println!("     Lines pending:       +{} / -{}",
                    pending_additions.to_string().green(),
                    pending_deletions.to_string().red()
                );
                println!();

                if !recent_reviews.is_empty() {
                    println!("  🕐 Recent Activity (last {} days):", days);
                    recent_reviews.sort_by(|a, b| b.1.cmp(&a.1));
                    for (title, date, _) in recent_reviews.iter().take(5) {
                        let days_ago = (Utc::now() - *date).num_days();
                        let when = if days_ago == 0 {
                            "today".green()
                        } else if days_ago == 1 {
                            "yesterday".normal()
                        } else {
                            format!("{} days ago", days_ago).dimmed()
                        };
                        println!("     • {} ({})", title.bold(), when);
                    }
                    println!();
                }

                println!("  💡 Tip: Use `--json` flag for machine-readable output");
                println!("{}", "─".repeat(45));
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

fn format_duration(d: chrono::Duration) -> String {
    let total_hours = d.num_hours();
    if total_hours < 24 {
        format!("{}h", total_hours)
    } else {
        let days = d.num_days();
        if days < 7 {
            format!("{}d", days)
        } else {
            let weeks = days / 7;
            format!("{}w", weeks)
        }
    }
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
