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

            // Filter out snoozed PRs (unless --pr is specified)
            let filtered: Vec<_> = if cli.pr.is_none() {
                let snooze_file = output_dir
                    .clone()
                    .unwrap_or_else(|| PathBuf::from("./reviews"))
                    .join(".snoozed.json");

                let now = chrono::Utc::now();
                let snoozed_prs: Vec<(String, u64)> = if snooze_file.exists() {
                    if let Ok(content) = std::fs::read_to_string(&snooze_file) {
                        if let Ok(entries) = serde_json::from_str::<Vec<serde_json::Value>>(&content) {
                            entries
                                .into_iter()
                                .filter_map(|e| {
                                    let repo = e.get("repo")?.as_str()?.to_string();
                                    let pr_number = e.get("pr_number")?.as_u64()?;
                                    let until_str = e.get("snoozed_until")?.as_str()?;
                                    if let Ok(until) = chrono::DateTime::parse_from_rfc3339(until_str) {
                                        if until.with_timezone(&chrono::Utc) > now {
                                            return Some((repo, pr_number));
                                        }
                                    }
                                    None
                                })
                                .collect()
                        } else {
                            Vec::new()
                        }
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                };

                let _snoozed_count = snoozed_prs.len();
                filtered
                    .into_iter()
                    .filter(|r| !snoozed_prs.iter().any(|(repo, num)| *num == r.pr_number && repo == &r.repo))
                    .inspect(|_| ())
                    .collect()
            } else {
                filtered
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

        Commands::Comment { pr_number, text } => {
            let target_pr = cli.pr.or(pr_number.clone());

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
                        "Select PR to comment on [e.g. 1 or 1,3 or 1-3] (q to quit):".bold()
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
                println!("No PR found to comment on.");
                return Ok(());
            }

            for review in &prs {
                print!(
                    "\n💬 Posting comment on #{} {}... ",
                    review.pr_number,
                    review.pr_title
                );
                io::stdout().flush()?;

                let client = octocrab::Octocrab::builder()
                    .personal_token(cfg.github_token.clone())
                    .build()?;

                let result = client
                    .issues(&cfg.github_org, &review.repo)
                    .create_comment(review.pr_number, &text)
                    .await;

                match result {
                    Ok(_) => {
                        println!("{}", "✅ Commented".green());
                        println!("   📝 {} ({})", review.pr_title, review.repo);
                        println!("   💬 \"{}\"", text.yellow());
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

        Commands::Approve { pr_number, message } => {
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
                        "Select PR to approve [e.g. 1 or 1,3 or 1-3] (q to quit):".bold()
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
                println!("No PR found to approve.");
                return Ok(());
            }

            for review in prs {
                print!(
                    "\n⏳ Approving #{} {}... ",
                    review.pr_number,
                    review.pr_title
                );
                io::stdout().flush()?;

                let client = octocrab::Octocrab::builder()
                    .personal_token(cfg.github_token.clone())
                    .build()?;

                // Get the latest commit SHA for this PR
                let pr_details = client
                    .pulls(&cfg.github_org, &review.repo)
                    .get(review.pr_number)
                    .await;

                let commit_id = match pr_details {
                    Ok(pr) => pr.head.sha.clone(),
                    Err(e) => {
                        println!("{}", "❌ Failed".red());
                        println!("   Error getting PR details: {}", e);
                        continue;
                    }
                };

                // Use the pull request review API to approve
                #[allow(deprecated)]
                let result = client
                    .pulls(&cfg.github_org, &review.repo)
                    .pull_number(review.pr_number)
                    .reviews()
                    .create_review(
                        commit_id,
                        message.clone().unwrap_or_else(|| "LGTM!".to_string()),
                        octocrab::models::pulls::ReviewAction::Approve,
                        Vec::new(),
                    )
                    .await;

                match result {
                    Ok(_) => {
                        println!("{}", "✅ Approved".green());
                        println!("   👍 You approved {} ({})", review.pr_title, review.repo);
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

        Commands::Claim { all, pr_numbers } => {
            let targets: Vec<_> = if all {
                if reviews.is_empty() {
                    println!("No pending reviews found.");
                    return Ok(());
                }
                reviews.clone()
            } else if let Some(ref nums) = pr_numbers {
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
                if reviews.is_empty() {
                    println!("No pending reviews found.");
                    return Ok(());
                }
                logger::print_reviews(&reviews, false);
                print!(
                    "\n{} ",
                    "Select PRs to claim [e.g. 1,3 or 1-3 or 'all'] (q to quit):".bold()
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
                println!("No PRs to claim.");
                return Ok(());
            }

            println!(
                "\n🎯 Claiming {} PR(s) for review...\n",
                targets.len().to_string().yellow().bold()
            );

            let mut success_count = 0u32;
            let mut fail_count = 0u32;

            for review in &targets {
                print!(
                    "  ⏳ #{} {}... ",
                    review.pr_number,
                    review.pr_title.dimmed()
                );
                io::stdout().flush()?;

                let client = octocrab::Octocrab::builder()
                    .personal_token(cfg.github_token.clone())
                    .build()?;

                match client
                    .pulls(&cfg.github_org, &review.repo)
                    .request_reviews(
                        review.pr_number,
                        vec![cfg.github_username.clone()],
                        Vec::<String>::new(),
                    )
                    .await
                {
                    Ok(_) => {
                        println!("{}", "✅ claimed".green());
                        success_count += 1;
                    }
                    Err(e) => {
                        println!("{}", "❌ failed".red());
                        println!("     Error: {}", e.to_string().dimmed());
                        fail_count += 1;
                    }
                }
            }

            println!();
            println!(
                "📊 Claimed {}/{} PRs",
                success_count.to_string().green(),
                targets.len().to_string().yellow()
            );
            if fail_count > 0 {
                println!(
                    "⚠️  {} PR(s) failed - may already be assigned or inaccessible",
                    fail_count.to_string().red()
                );
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

        Commands::Search { query, priority, json } => {
            let query_lower = query.to_lowercase();
            let filtered: Vec<_> = reviews
                .iter()
                .filter(|r| r.pr_title.to_lowercase().contains(&query_lower))
                .cloned()
                .collect();

            if filtered.is_empty() {
                println!("\n🔍 No reviews matching '{}' found.\n", query.yellow());
                return Ok(());
            }

            if json {
                let json_output = serde_json::to_string_pretty(&filtered)?;
                println!("{}", json_output);
            } else {
                println!(
                    "\n🔍 {} review(s) matching '{}'\n",
                    filtered.len().to_string().yellow().bold(),
                    query.yellow().bold()
                );
                logger::print_reviews(&filtered, priority);
            }
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

        Commands::Labels { pr_numbers, all, filter_by, json } => {
            let targets: Vec<_> = if all {
                if reviews.is_empty() {
                    println!("No pending reviews found.");
                    return Ok(());
                }
                reviews.clone()
            } else if let Some(ref nums) = pr_numbers {
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
                if reviews.is_empty() {
                    println!("No pending reviews found.");
                    return Ok(());
                }
                logger::print_reviews(&reviews, false);
                print!(
                    "\n{} ",
                    "Select PRs to show labels [e.g. 1,3 or 1-3 or 'all'] (q to quit):".bold()
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

            // Collect all labels with their PRs
            let mut all_labels_data: Vec<(github::PendingReview, Vec<github::PullRequestLabel>)> = Vec::new();
            let mut total_labels_count = 0usize;

            for review in &targets {
                print!("\n⏳ Fetching labels for #{} {}... ", review.pr_number, review.pr_title);
                io::stdout().flush()?;

                match github::fetch_pr_labels(
                    &cfg.github_token,
                    &cfg.github_org,
                    &review.repo,
                    review.pr_number,
                )
                .await
                {
                    Ok(labels) => {
                        println!("{}", "done".green());
                        total_labels_count += labels.len();
                        all_labels_data.push((review.clone(), labels));
                    }
                    Err(e) => {
                        println!("{}", "failed".red());
                        println!("  ❌ Error fetching labels: {}", e);
                    }
                }
            }

            if json {
                #[derive(serde::Serialize)]
                struct LabelOutput<'a> {
                    pr_number: u64,
                    pr_title: &'a str,
                    repo: &'a str,
                    pr_url: &'a str,
                    labels: &'a [github::PullRequestLabel],
                }
                let json_output: Vec<LabelOutput> = all_labels_data
                    .iter()
                    .map(|(r, l)| LabelOutput {
                        pr_number: r.pr_number,
                        pr_title: &r.pr_title,
                        repo: &r.repo,
                        pr_url: &r.pr_url,
                        labels: l,
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&json_output).unwrap_or_default());
            } else {
                println!("\n🏷️  Labels Summary\n{}", "─".repeat(45));

                // If filter_by is specified, show only matching labels
                if let Some(ref filter) = filter_by {
                    let filter_lower = filter.to_lowercase();
                    let mut found_any = false;
                    for (review, labels) in &all_labels_data {
                        for label in labels {
                            if label.name.to_lowercase().contains(&filter_lower) {
                                found_any = true;
                                let _color_hex = format!("#{}", label.color);
                                println!(
                                    "  {}  {}  (on #{} - {})",
                                    colorize_label(&label.name, &label.color),
                                    review.pr_title.bold(),
                                    review.pr_number,
                                    review.repo.dimmed()
                                );
                            }
                        }
                    }
                    if !found_any {
                        println!("  No labels matching '{}' found.", filter.yellow());
                    }
                } else {
                    // Show all labels grouped by PR
                    for (review, labels) in &all_labels_data {
                        println!("\n📄 #{} {} ({})", review.pr_number, review.pr_title.bold(), review.repo);
                        if labels.is_empty() {
                            println!("  (no labels)");
                        } else {
                            for label in labels {
                                println!(
                                    "  {}  {}",
                                    colorize_label(&label.name, &label.color),
                                    label.description.as_ref().map(|d| d.as_str()).unwrap_or("").dimmed()
                                );
                            }
                        }
                    }

                    // Show label frequency summary
                    if total_labels_count > 0 {
                        use std::collections::HashMap;
                        let mut label_counts: HashMap<String, (String, usize)> = HashMap::new();
                        for (_, labels) in &all_labels_data {
                            for label in labels {
                                let entry = label_counts.entry(label.name.clone()).or_insert_with(|| (label.color.clone(), 0));
                                entry.1 += 1;
                            }
                        }
                        println!("\n📊 Label Frequency:");
                        let mut sorted: Vec<_> = label_counts.iter().collect();
                        sorted.sort_by(|a, b| b.1.1.cmp(&a.1.1));
                        for (name, (color, count)) in sorted.iter().take(10) {
                            let bar = "█".repeat(*count).cyan();
                            println!("  {}  {}  {}", colorize_label(name, color), bar, count);
                        }
                    }
                }
                println!("{}", "─".repeat(45));
            }
        }

        Commands::Review { pr_number, context, output_file, language } => {
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
                        "Select PR to review [e.g. 1 or 1,3 or 1-3] (q to quit):".bold()
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
                println!("No PR found to review.");
                return Ok(());
            }

            for review in prs {
                print!(
                    "\n⏳ Fetching diff for #{} {}... ",
                    review.pr_number,
                    review.pr_title
                );
                io::stdout().flush()?;

                match github::fetch_pr_diff(
                    &cfg.github_token,
                    &cfg.github_org,
                    &review.repo,
                    review.pr_number,
                )
                .await
                {
                    Ok(files) => {
                        println!("{}", "done".green());

                        let total_additions: u64 = files.iter().map(|f| f.additions).sum();
                        let total_deletions: u64 = files.iter().map(|f| f.deletions).sum();

                        println!("\n{}", "─".repeat(60));
                        println!("📄 {}  #{}", review.pr_title.bold(), review.pr_number);
                        println!("   👤 {}  •  📁 {}  •  +{} / -{} lines",
                            review.pr_author.cyan(),
                            review.repo,
                            total_additions.to_string().green(),
                            total_deletions.to_string().red()
                        );
                        println!("{}", "─".repeat(60));

                        if files.is_empty() {
                            println!("  (no file changes)");
                        } else {
                            // Build unified diff output
                            let mut unified_diff = String::new();
                            for file in &files {
                                let status_icon = match file.status.as_str() {
                                    "added" => "+",
                                    "removed" => "-",
                                    "modified" => "M",
                                    "renamed" => "R",
                                    _ => "?",
                                };
                                let lang = language.as_ref().or(file.language.as_ref()).map(|s| s.as_str()).unwrap_or("");
                                let header = format!(
                                    "diff --git a/{} b/{} {}",
                                    file.filename, file.filename,
                                    if file.status == "renamed" { "(renamed)" } else { "" }
                                );
                                let hunk_header = if file.patch.is_some() {
                                    format!(
                                        "@@ -{},{} +{},{} @@ [{}] {}",
                                        1, file.deletions,
                                        1, file.additions,
                                        lang,
                                        status_icon
                                    )
                                } else {
                                    format!(
                                        "@@ -0,0 +0,0 @@ [{}] {}",
                                        lang,
                                        status_icon
                                    )
                                };

                                unified_diff.push_str(&format!("{}\n", header));
                                unified_diff.push_str(&format!("{}{}\n", hunk_header, status_icon));

                                if let Some(ref patch) = file.patch {
                                    // Normalize patch context lines
                                    let context_str = " ".repeat(context as usize);
                                    for line in patch.lines() {
                                        let line = line.trim_end();
                                        if line.is_empty() {
                                            unified_diff.push_str(&format!("{}{}\n", context_str, line));
                                        } else if line.starts_with('+') && !line.starts_with("+++") {
                                            unified_diff.push_str(&format!("{}{}\n", "+".yellow(), &line[1..]));
                                        } else if line.starts_with('-') && !line.starts_with("---") {
                                            unified_diff.push_str(&format!("{}{}\n", "-".red(), &line[1..]));
                                        } else if line.starts_with(' ') || line.starts_with("@@") {
                                            unified_diff.push_str(&format!(" {}\n", line));
                                        } else {
                                            unified_diff.push_str(&format!("{}\n", line));
                                        }
                                    }
                                } else {
                                    unified_diff.push_str(&format!(
                                        "  (binary or no preview available)\n"
                                    ));
                                }
                                unified_diff.push_str("\n");
                            }

                            if let Some(ref path) = output_file {
                                std::fs::write(path, &unified_diff)?;
                                println!("   💾 Diff written to {} ({:.1} KB)",
                                    path.display().to_string().cyan(),
                                    unified_diff.len() as f64 / 1024.0
                                );
                            } else {
                                // Try to use bat for syntax highlighting, fallback to plain print
                                let use_bat = std::process::Command::new("which")
                                    .arg("bat")
                                    .output()
                                    .map(|o| o.status.success())
                                    .unwrap_or(false);

                                if use_bat {
                                    // Pipe diff through bat with appropriate language
                                    let mut cmd = std::process::Command::new("bat");
                                    cmd.arg("--style=changes")
                                       .arg("--color=always")
                                       .arg("--language=diff");
                                    cmd.arg("--");
                                    cmd.arg("-");

                                    match cmd.stdin(std::process::Stdio::piped()).spawn() {
                                        Ok(child) => {
                                            use std::io::Write;
                                            if let Some(ref stdin) = child.stdin {
                                                let mut w = stdin;
                                                let _ = w.write_all(unified_diff.as_bytes());
                                            }
                                            let _ = child.wait_with_output();
                                        }
                                        Err(_) => {
                                            println!("{}", unified_diff);
                                        }
                                    }
                                } else {
                                    // Fallback: print with basic coloring
                                    println!("\n{}\n", unified_diff);
                                }
                            }
                        }
                        println!("{}", "─".repeat(60));
                    }
                    Err(e) => {
                        println!("{}", "failed".red());
                        println!("  ❌ Error fetching diff: {}", e);
                    }
                }
            }
            println!();
        }

        Commands::Top { limit, min_score, json } => {
            let limit = limit.unwrap_or(10);
            let min_score = min_score.unwrap_or(3).min(5);

            // Calculate priority for all reviews
            let mut scored: Vec<_> = reviews
                .iter()
                .map(|r| {
                    let score = logger::calculate_priority_score(r);
                    (r.clone(), score)
                })
                .filter(|(_, score)| *score >= min_score)
                .collect();

            // Sort by priority score descending, then by age ascending
            scored.sort_by(|a, b| {
                let score_cmp = b.1.cmp(&a.1);
                if score_cmp == std::cmp::Ordering::Equal {
                    a.0.created_at.cmp(&b.0.created_at)
                } else {
                    score_cmp
                }
            });

            let top_prs: Vec<_> = scored.into_iter().take(limit).collect();

            if top_prs.is_empty() {
                println!("\n🎯 No high-priority reviews found (min score: {})\n", min_score);
                return Ok(());
            }

            if json {
                #[derive(serde::Serialize)]
                struct TopReview<'a> {
                    repo: &'a str,
                    pr_number: u64,
                    pr_title: &'a str,
                    pr_author: &'a str,
                    pr_url: &'a str,
                    priority_score: u8,
                    age_days: i64,
                    additions: u64,
                    deletions: u64,
                    draft: bool,
                }
                let output: Vec<TopReview> = top_prs
                    .iter()
                    .map(|(r, score)| TopReview {
                        repo: &r.repo,
                        pr_number: r.pr_number,
                        pr_title: &r.pr_title,
                        pr_author: &r.pr_author,
                        pr_url: &r.pr_url,
                        priority_score: *score,
                        age_days: (chrono::Utc::now() - r.created_at).num_days(),
                        additions: r.additions,
                        deletions: r.deletions,
                        draft: r.draft,
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!(
                    "\n🎯 Top {} Priority Reviews (score >= {})\n{}",
                    limit,
                    min_score,
                    "─".repeat(50)
                );
                for (r, score) in &top_prs {
                    let age_days = (chrono::Utc::now() - r.created_at).num_days();
                    let age_label = match age_days {
                        0 => "today".green(),
                        1 => "1 day".normal(),
                        _ => format!("{} days", age_days).red(),
                    };

                    let size = r.additions + r.deletions;
                    let size_label = if size < 50 {
                        "XS".green()
                    } else if size < 200 {
                        "S".green()
                    } else if size < 500 {
                        "M".yellow()
                    } else if size < 1000 {
                        "L".red()
                    } else {
                        "XL".magenta()
                    };

                    let draft_label = if r.draft { " [DRAFT]".yellow() } else { "".normal() };

                    println!(
                        "  ⭐{}  {}  #{} ({}){}",
                        score,
                        r.pr_title.bold(),
                        r.pr_number,
                        r.repo.dimmed(),
                        draft_label
                    );
                    println!(
                        "      👤 {}  •  📦 {} ({} lines)  •  ⏱️ {}",
                        r.pr_author.cyan(),
                        size_label,
                        size,
                        age_label
                    );
                    println!(
                        "      🔗 {}",
                        r.pr_url.blue().underline()
                    );
                    println!();
                }
                println!("{}", "─".repeat(50));
                println!("  💡 Use `--min-score 4` for only critical PRs");
                println!("  💡 Use `--json` for scripting\n");
            }
        }

        Commands::Quick { max_lines, limit, json } => {
            let max_lines = max_lines.unwrap_or(200);
            let limit = limit.unwrap_or(10);

            // Filter to small, non-draft PRs
            let quick_wins: Vec<_> = reviews
                .iter()
                .filter(|r| !r.draft && (r.additions + r.deletions) <= max_lines)
                .take(limit)
                .cloned()
                .collect();

            if quick_wins.is_empty() {
                println!("\n⚡ No quick wins found (max {} lines, non-draft)\n", max_lines);
                return Ok(());
            }

            if json {
                #[derive(serde::Serialize)]
                struct QuickWin<'a> {
                    repo: &'a str,
                    pr_number: u64,
                    pr_title: &'a str,
                    pr_author: &'a str,
                    pr_url: &'a str,
                    age_days: i64,
                    additions: u64,
                    deletions: u64,
                }
                let output: Vec<QuickWin> = quick_wins
                    .iter()
                    .map(|r| QuickWin {
                        repo: &r.repo,
                        pr_number: r.pr_number,
                        pr_title: &r.pr_title,
                        pr_author: &r.pr_author,
                        pr_url: &r.pr_url,
                        age_days: (chrono::Utc::now() - r.created_at).num_days(),
                        additions: r.additions,
                        deletions: r.deletions,
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!(
                    "\n⚡ Quick Wins (≤{} lines, non-draft)\n{}",
                    max_lines,
                    "─".repeat(50)
                );
                for r in &quick_wins {
                    let age_days = (chrono::Utc::now() - r.created_at).num_days();
                    let age_label = match age_days {
                        0 => "today".green(),
                        1 => "1 day".normal(),
                        _ => format!("{} days", age_days).red(),
                    };
                    let total = r.additions + r.deletions;
                    let size_label = if total < 50 {
                        format!("{} lines", total).green()
                    } else {
                        format!("{} lines", total).yellow()
                    };

                    println!(
                        "  ⚡ {} #{} ({})\n      👤 {}  •  📦 {}  •  ⏱️ {}\n      🔗 {}",
                        r.pr_title.bold(),
                        r.pr_number,
                        r.repo.dimmed(),
                        r.pr_author.cyan(),
                        size_label,
                        age_label,
                        r.pr_url.blue().underline()
                    );
                    println!();
                }
                println!("{}", "─".repeat(50));
                println!("  💡 Use `--max-lines 100` for tiny PRs only");
                println!("  💡 Use `--json` for scripting\n");
            }
        }

        Commands::Catchup { min_age, limit, json } => {
            let min_age = min_age as i64;
            let limit = limit.unwrap_or(10);

            let now = chrono::Utc::now();
            let cutoff = now - chrono::Duration::days(min_age);

            // Filter: only PRs older than min_age, sorted oldest-first
            let mut neglected: Vec<_> = reviews
                .iter()
                .filter(|r| r.created_at <= cutoff)
                .cloned()
                .collect();
            neglected.sort_by_key(|r| r.created_at); // oldest first

            let shown: Vec<_> = neglected.iter().take(limit).cloned().collect();

            if shown.is_empty() {
                println!("\n🎯 No neglected PRs found (all younger than {} days)\n", min_age);
                return Ok(());
            }

            if json {
                #[derive(serde::Serialize)]
                struct CatchupItem<'a> {
                    repo: &'a str,
                    pr_number: u64,
                    pr_title: &'a str,
                    pr_author: &'a str,
                    pr_url: &'a str,
                    age_days: i64,
                    additions: u64,
                    deletions: u64,
                    draft: bool,
                    neglect_score: u8,
                }
                let output: Vec<CatchupItem> = shown
                    .iter()
                    .map(|r| {
                        let age_days = (now - r.created_at).num_days();
                        let neglect_score = logger::calculate_priority_score(r);
                        CatchupItem {
                            repo: &r.repo,
                            pr_number: r.pr_number,
                            pr_title: &r.pr_title,
                            pr_author: &r.pr_author,
                            pr_url: &r.pr_url,
                            age_days,
                            additions: r.additions,
                            deletions: r.deletions,
                            draft: r.draft,
                            neglect_score,
                        }
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                let total_neglected = neglected.len();
                println!(
                    "\n🎯 Catchup — {} PR(s) older than {} days, showing {} oldest\n{}",
                    total_neglected.to_string().yellow().bold(),
                    min_age,
                    limit,
                    "─".repeat(50)
                );

                for r in &shown {
                    let age_days = (now - r.created_at).num_days();
                    let neglect_score = logger::calculate_priority_score(r);

                    // Visual urgency bar
                    let bar_len = (neglect_score as usize).min(5);
                    let urgency_bar: String = (0..5)
                        .map(|i| {
                            if i < bar_len {
                                if i < 2 { "🔵" } else if i < 4 { "🟡" } else { "🔴" }
                            } else {
                                "⚪"
                            }
                        })
                        .collect();

                    let age_label = if age_days == 0 {
                        "today".green()
                    } else if age_days == 1 {
                        "1 day".normal()
                    } else if age_days <= 7 {
                        format!("{} days", age_days).yellow()
                    } else {
                        format!("{} days!", age_days).red()
                    };

                    let total = r.additions + r.deletions;
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

                    let draft_str = if r.draft { " 📝 DRAFT" } else { "" };

                    println!(
                        "  {}  {} #{} ({}){}\n      👤 {}  •  📦 {} ({} lines)  •  ⏱️ {} old",
                        urgency_bar,
                        r.pr_title.bold(),
                        r.pr_number,
                        r.repo.dimmed(),
                        draft_str.yellow(),
                        r.pr_author.cyan(),
                        size_label,
                        total,
                        age_label
                    );
                    println!(
                        "      🔗 {}",
                        r.pr_url.blue().underline()
                    );
                    println!();
                }

                if total_neglected > limit {
                    println!(
                        "  ...and {} more. Use `--limit 20` to see additional.\n",
                        total_neglected - limit
                    );
                }
                println!("{}", "─".repeat(50));
                println!("  💡 Use `--min-age 7` to focus on week-old+ PRs");
                println!("  💡 Use `--json` for scripting\n");
            }
        }

        Commands::Age { min_days, older_than, grouped, json } => {
            use chrono::Utc;

            let now = Utc::now();

            // Age buckets: (label, emoji, max_days, min_days)
            // None for max means +infinity
            #[derive(Clone, Copy)]
            struct Bucket(&'static str, &'static str, Option<i64>, Option<i64>);
            const BUCKETS: [Bucket; 5] = [
                Bucket("Overdue",     "💀", Some(14), Some(15)),
                Bucket("Stale",       "🔥", Some(7),  Some(8)),
                Bucket("Aging",       "⏳", Some(3),  Some(4)),
                Bucket("Fresh",       "🌱", Some(1),  Some(2)),
                Bucket("New",         "🆕", None,      Some(0)),
            ];

            let min_days = min_days.map(|d| d as i64);
            let older_than = older_than.map(|d| d as i64);

            #[derive(serde::Serialize)]
            struct AgeItem<'a> {
                repo: &'a str,
                pr_number: u64,
                pr_title: &'a str,
                pr_author: &'a str,
                pr_url: &'a str,
                age_days: i64,
                additions: u64,
                deletions: u64,
                draft: bool,
            }

            #[derive(serde::Serialize)]
            struct AgeBucket<'a> {
                label: &'a str,
                emoji: &'a str,
                prs: Vec<AgeItem<'a>>,
            }

            let mut buckets: Vec<(Bucket, Vec<&github::PendingReview>)> =
                BUCKETS.iter().cloned().map(|b| (b, vec![])).collect();

            for r in &reviews {
                let age_days = (now - r.created_at).num_days();

                // Apply --older-than filter
                if let Some(cutoff) = older_than {
                    if age_days <= cutoff {
                        continue;
                    }
                }

                // Apply --min-days filter
                if let Some(min) = min_days {
                    if age_days < min {
                        continue;
                    }
                }

                // Find matching bucket (last match wins since ranges overlap)
                let mut matched = false;
                for (bucket, prs) in &mut buckets {
                    let Bucket(_, _, bucket_max, bucket_min) = *bucket;
                    let in_bucket = match (bucket_min, bucket_max) {
                        (Some(min), Some(max)) => age_days >= min && age_days <= max,
                        (Some(min), None) => age_days >= min,
                        (None, Some(max)) => age_days <= max,
                        (None, None) => true,
                    };
                    if in_bucket {
                        prs.push(r);
                        matched = true;
                    }
                }
                let _ = matched; // suppress unused warning
            }

            if json {
                let output: Vec<AgeBucket> = buckets
                    .iter()
                    .filter(|(_, prs)| !prs.is_empty())
                    .map(|(bucket, prs)| {
                        let Bucket(label, emoji, _, _) = *bucket;
                        AgeBucket {
                            label,
                            emoji,
                            prs: prs.iter().map(|r| {
                                let age_days = (now - r.created_at).num_days();
                                AgeItem {
                                    repo: &r.repo,
                                    pr_number: r.pr_number,
                                    pr_title: &r.pr_title,
                                    pr_author: &r.pr_author,
                                    pr_url: &r.pr_url,
                                    age_days,
                                    additions: r.additions,
                                    deletions: r.deletions,
                                    draft: r.draft,
                                }
                            }).collect(),
                        }
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else if grouped {
                // Grouped view: one section per bucket
                let total: usize = buckets.iter().map(|(_, p)| p.len()).sum();
                println!("\n📊 Age Breakdown — {} PRs total\n{}", total, "─".repeat(50));

                let mut any_shown = false;
                for (bucket, prs) in &buckets {
                    if prs.is_empty() {
                        continue;
                    }
                    any_shown = true;
                    let Bucket(label, emoji, _, _) = *bucket;
                    println!("\n{} {} ({} PRs)", emoji, label.bold(), prs.len());
                    println!("{}", "─".repeat(40));

                    for r in prs {
                        let age_days = (now - r.created_at).num_days();
                        let age_str = if age_days == 0 {
                            "today".green().to_string()
                        } else if age_days == 1 {
                            "1 day".yellow().to_string()
                        } else {
                            format!("{} days", age_days).red().to_string()
                        };
                        let draft_str = if r.draft { " 📝DRAFT".yellow().to_string() } else { String::new() };
                        let _total_lines = r.additions + r.deletions;
                        println!(
                            "  #{}  {}  •  👤 {}  •  +{}/-{} lines  •  {} old{}\n      📁 {}  🔗 {}",
                            r.pr_number,
                            r.pr_title.bold(),
                            r.pr_author.cyan(),
                            r.additions,
                            r.deletions,
                            age_str,
                            draft_str,
                            r.repo.dimmed(),
                            r.pr_url.blue().underline()
                        );
                    }
                }

                if !any_shown {
                    println!("\n  No PRs match the specified age filters.\n");
                }
                println!("\n{}", "─".repeat(50));
                println!("  💡 Use `--older-than 7` to see only week-old+ PRs");
                println!("  💡 Use `--json` for scripting\n");
            } else {
                // Flat view: sorted oldest-first within each bucket, buckets ordered newest→oldest
                let mut all_filtered: Vec<&github::PendingReview> = Vec::new();
                for (_, prs) in &buckets {
                    all_filtered.extend(prs.iter().cloned());
                }
                // Sort oldest-first (most neglected first)
                all_filtered.sort_by_key(|r| r.created_at);

                if all_filtered.is_empty() {
                    println!("\n⏰ No PRs match the specified age filters.\n");
                    return Ok(());
                }

                println!(
                    "\n⏰ Age Report — {} PRs (oldest first)\n{}",
                    all_filtered.len(),
                    "─".repeat(50)
                );

                for r in &all_filtered {
                    let age_days = (now - r.created_at).num_days();

                    // Determine bucket for emoji
                    let (emoji, bucket_label) = if age_days >= 15 {
                        ("💀", "Overdue")
                    } else if age_days >= 8 {
                        ("🔥", "Stale")
                    } else if age_days >= 4 {
                        ("⏳", "Aging")
                    } else if age_days >= 2 {
                        ("🌱", "Fresh")
                    } else {
                        ("🆕", "New")
                    };

                    let age_str = if age_days == 0 {
                        "today".green().to_string()
                    } else if age_days == 1 {
                        "1 day".yellow().to_string()
                    } else if age_days <= 7 {
                        format!("{} days", age_days).yellow().to_string()
                    } else {
                        format!("{} days", age_days).red().to_string()
                    };

                    let draft_str = if r.draft { " 📝DRAFT".yellow().to_string() } else { String::new() };

                    println!(
                        "{}  {}  #{} ({})\n    👤 {}  •  +{}/-{} lines  •  {} old{}",
                        emoji,
                        bucket_label.cyan(),
                        r.pr_number,
                        r.repo.dimmed(),
                        r.pr_author.cyan(),
                        r.additions,
                        r.deletions,
                        age_str,
                        draft_str
                    );
                    println!("    🔗 {}", r.pr_url.blue().underline());
                    println!();
                }

                println!("{}", "─".repeat(50));
                println!("  Buckets: 🆕 New <2d  🌱 Fresh 2-3d  ⏳ Aging 4-7d  🔥 Stale 8-14d  💀 Overdue 15d+");
                println!("  💡 Use `--grouped` to see PRs organized by age bucket");
                println!("  💡 Use `--older-than 7` to focus on week-old+ PRs\n");
            }
        }

        Commands::Snooze { action, pr_numbers, days } => {
            use serde::{Deserialize, Serialize};

            // Snooze storage file
            let snooze_file = output_dir
                .clone()
                .unwrap_or_else(|| PathBuf::from("./reviews"))
                .join(".snoozed.json");

            #[derive(Debug, Clone, Serialize, Deserialize, Default)]
            struct SnoozeEntry {
                pub repo: String,
                pub pr_number: u64,
                pub pr_title: String,
                pub snoozed_until: String, // ISO 8601 timestamp
            }

            // Load existing snooze data
            let mut snoozed: Vec<SnoozeEntry> = if snooze_file.exists() {
                if let Ok(content) = std::fs::read_to_string(&snooze_file) {
                    serde_json::from_str(&content).unwrap_or_default()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };

            match action {
                cli::SnoozeAction::Add => {
                    let duration_days = days.unwrap_or(3) as i64;
                    let snooze_until = (chrono::Utc::now() + chrono::Duration::days(duration_days))
                        .to_rfc3339();

                    // If no PR numbers provided, show interactive picker
                    let targets: Vec<_> = if let Some(ref nums) = pr_numbers {
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
                        if reviews.is_empty() {
                            println!("No pending reviews found to snooze.");
                            return Ok(());
                        }
                        logger::print_reviews(&reviews, false);
                        print!(
                            "\n{} ",
                            "Select PRs to snooze [e.g. 1,3 or 1-3 or 'all'] (q to quit):".bold()
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
                        println!("No PRs to snooze.");
                        return Ok(());
                    }

                    println!(
                        "\n😴 Snoozing {} PR(s) for {} day(s)...\n",
                        targets.len().to_string().yellow().bold(),
                        duration_days.to_string().cyan()
                    );

                    for review in &targets {
                        // Remove existing entry if present (to update snooze time)
                        snoozed.retain(|e| !(e.repo == review.repo && e.pr_number == review.pr_number));

                        snoozed.push(SnoozeEntry {
                            repo: review.repo.clone(),
                            pr_number: review.pr_number,
                            pr_title: review.pr_title.clone(),
                            snoozed_until: snooze_until.clone(),
                        });

                        println!(
                            "  😴 {} ({}) - until {}",
                            review.pr_title.dimmed(),
                            format!("#{}", review.pr_number).dimmed(),
                            snooze_until[..10].dimmed()
                        );
                    }

                    // Save snooze data
                    if let Some(ref dir) = output_dir {
                        std::fs::create_dir_all(dir).ok();
                    }
                    if let Err(e) = std::fs::write(&snooze_file, serde_json::to_string_pretty(&snoozed)?) {
                        println!("  ⚠️ Failed to save snooze data: {}", e);
                    } else {
                        println!("\n✅ Snooze list saved ({} PRs snoozed)", snoozed.len());
                    }
                    println!();
                }

                cli::SnoozeAction::List => {
                    let now = chrono::Utc::now();
                    let mut active: Vec<_> = snoozed
                        .iter()
                        .filter(|e| {
                            if let Ok(until) = chrono::DateTime::parse_from_rfc3339(&e.snoozed_until) {
                                until.with_timezone(&chrono::Utc) > now
                            } else {
                                false
                            }
                        })
                        .collect();

                    if active.is_empty() {
                        println!("\n😴 No currently snoozed PRs.\n");
                        return Ok(());
                    }

                    println!(
                        "\n😴 Currently Snoozed PRs ({} total)\n{}",
                        active.len(),
                        "─".repeat(50)
                    );

                    // Sort by expiry time
                    active.sort_by(|a, b| {
                        let a_time = chrono::DateTime::parse_from_rfc3339(&a.snoozed_until).map(|t| t.timestamp()).unwrap_or(0);
                        let b_time = chrono::DateTime::parse_from_rfc3339(&b.snoozed_until).map(|t| t.timestamp()).unwrap_or(0);
                        a_time.cmp(&b_time)
                    });

                    for entry in &active {
                        let until = chrono::DateTime::parse_from_rfc3339(&entry.snoozed_until)
                            .map(|t| t.with_timezone(&chrono::Utc))
                            .unwrap_or(now);
                        let remaining = (until - now).num_hours();
                        let remaining_label = if remaining < 24 {
                            format!("{}h left", remaining).red()
                        } else {
                            format!("{}d left", remaining / 24).yellow()
                        };

                        println!(
                            "  😴 {}  #{} ({}) - {}",
                            entry.pr_title.bold(),
                            entry.pr_number,
                            entry.repo.dimmed(),
                            remaining_label
                        );
                    }
                    println!("{}", "─".repeat(50));
                    println!("\n💡 Use `snooze remove --pr 123` to wake a PR early\n");
                }

                cli::SnoozeAction::Remove => {
                    if let Some(ref nums) = pr_numbers {
                        let to_remove: Vec<u64> = nums
                            .split(',')
                            .filter_map(|p| p.trim().parse().ok())
                            .collect();

                        if to_remove.is_empty() {
                            println!("❌ No valid PR numbers provided.");
                            return Ok(());
                        }

                        let initial_len = snoozed.len();
                        snoozed.retain(|e| !to_remove.contains(&e.pr_number));

                        let removed = initial_len - snoozed.len();
                        if removed > 0 {
                            if let Err(e) = std::fs::write(&snooze_file, serde_json::to_string_pretty(&snoozed)?) {
                                println!("  ⚠️ Failed to save snooze data: {}", e);
                            } else {
                                println!("\n✅ Removed {} PR(s) from snooze list ({} remaining)", removed, snoozed.len());
                            }
                        } else {
                            println!("\n😶 No matching snoozed PRs found.");
                        }
                    } else {
                        println!("\n❌ Please specify PR numbers to remove: `snooze remove --pr 123,456`\n");
                    }
                }

                cli::SnoozeAction::Clear => {
                    if snoozed.is_empty() {
                        println!("\n😴 Snooze list is already empty.\n");
                        return Ok(());
                    }

                    let count = snoozed.len();
                    snoozed.clear();
                    if snooze_file.exists() {
                        std::fs::remove_file(&snooze_file).ok();
                    }
                    println!("\n🧹 Cleared {} snoozed PR(s) from the list.\n", count);
                }
            }

            // If listing/showing reviews, filter out snoozed PRs
            // (The actual filtering happens in the List command below via a shared helper)
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

        Commands::Activity { days, json } => {
            println!("\n📈 Fetching your review activity (last {} days)...\n", days);

            match github::fetch_my_review_activity(
                &cfg.github_token,
                &cfg.github_org,
                &cfg.github_repos,
                &cfg.github_username,
                days,
            )
            .await
            {
                Ok(activities) => {
                    if json {
                        println!("{}", serde_json::to_string_pretty(&activities)?);
                    } else {
                        println!("📊 Your Review Activity  (last {} days)\n{}", days, "─".repeat(45));
                        println!("  Total PRs reviewed:  {}", activities.len());

                        if activities.is_empty() {
                            println!("\n  😴 No review activity found in this period.\n");
                        } else {
                            // Group by day
                            use std::collections::HashMap;
                            let mut by_day: HashMap<String, Vec<_>> = HashMap::new();
                            for activity in &activities {
                                let day = activity.reviewed_at.format("%Y-%m-%d").to_string();
                                by_day.entry(day).or_insert_with(Vec::new).push(activity);
                            }

                            // Show by-day breakdown
                            let mut days_sorted: Vec<_> = by_day.keys().collect();
                            days_sorted.sort_by(|a, b| b.cmp(a)); // newest first

                            for day in days_sorted {
                                let items = by_day.get(day).unwrap();

                                // Count by state
                                let approved = items.iter().filter(|a| a.state.contains("APPROVED")).count();
                                let changes_req = items.iter().filter(|a| a.state.contains("CHANGES_REQUESTED")).count();
                                let commented = items.iter().filter(|a| a.state.contains("COMMENT")).count();

                                let day_label = if *day == chrono::Utc::now().format("%Y-%m-%d").to_string() {
                                    "today".green().bold()
                                } else if *day == (chrono::Utc::now() - chrono::Duration::days(1)).format("%Y-%m-%d").to_string() {
                                    "yesterday".normal().bold()
                                } else {
                                    day.yellow()
                                };

                                println!("\n  📅 {}  ({} PRs)", day_label, items.len());
                                if approved > 0 { print!("    ✅ {} approved", approved); }
                                if changes_req > 0 { print!("    🔁 {} changes requested", changes_req); }
                                if commented > 0 { print!("    💬 {} commented", commented); }
                                println!();

                                for activity in items.iter().take(5) {
                                    let state_icon = if activity.state.contains("APPROVED") {
                                        "✅".to_string()
                                    } else if activity.state.contains("CHANGES_REQUESTED") {
                                        "🔁".to_string()
                                    } else {
                                        "💬".to_string()
                                    };
                                    let title = if activity.pr_title.len() > 50 {
                                        format!("{}...", &activity.pr_title[..47])
                                    } else {
                                        activity.pr_title.clone()
                                    };
                                    println!(
                                        "    {}  #{}  {} ({})",
                                        state_icon,
                                        activity.pr_number,
                                        title.dimmed(),
                                        activity.repo.dimmed()
                                    );
                                }
                                if items.len() > 5 {
                                    println!("    ... and {} more", items.len() - 5);
                                }
                            }

                            // Summary by repo
                            let mut by_repo: HashMap<String, usize> = HashMap::new();
                            for activity in &activities {
                                *by_repo.entry(activity.repo.clone()).or_insert(0) += 1;
                            }
                            println!("\n  📁 By repository:");
                            let mut sorted: Vec<_> = by_repo.iter().collect();
                            sorted.sort_by(|a, b| b.1.cmp(a.1));
                            for (repo, count) in sorted.iter().take(5) {
                                println!("    {}: {}", repo, count);
                            }
                        }
                        println!("{}", "─".repeat(45));
                        println!("\n  💡 Use `--json` for machine-readable output\n");
                    }
                }
                Err(e) => {
                    println!("{}", "❌ Failed to fetch activity".red());
                    println!("   Error: {}", e);
                }
            }
        }

        Commands::Mentions { unread_only, limit, json } => {
            let limit = limit.unwrap_or(20);

            println!("\n🔔 Fetching your GitHub notifications...\n");

            match github::fetch_mentions(
                &cfg.github_token,
                &cfg.github_username,
                unread_only,
                limit,
            )
            .await
            {
                Ok(mentions) => {
                    if mentions.is_empty() {
                        if json {
                            println!("{}", serde_json::to_string_pretty(&serde_json::json!([]))?);
                        } else {
                            println!("  😴 No notifications found.");
                            if unread_only {
                                println!("  (showing all, not just unread — use `-u` to filter)");
                            }
                        }
                        return Ok(());
                    }

                    let total = mentions.len();
                    let unread_count = mentions.iter().filter(|m| m.unread).count();

                    if json {
                        println!("{}", serde_json::to_string_pretty(&mentions)?);
                    } else {
                        println!("🔔 Notifications  ({} total{} | showing top {})\n{}",
                            total,
                            if unread_count > 0 { format!(", {} unread", unread_count) } else { String::new() },
                            limit,
                            "─".repeat(50)
                        );

                        for (i, mention) in mentions.iter().enumerate() {
                            let reason_label = match mention.reason.as_str() {
                                "mention" => "🏷️ mentioned",
                                "review_requested" => "👀 review requested",
                                "assign" => "📌 assigned",
                                "author" => "✍️ you authored",
                                "team_mention" => "👥 team mentioned",
                                "cm" => "💬 comment",
                                "subscribed" => "📬 subscribed",
                                _ => "📌 unknown",
                            };

                            let unread_marker = if mention.unread { " 🔵" } else { "" };
                            let age_days = (chrono::Utc::now() - mention.updated_at).num_days();
                            let age_label = if age_days == 0 {
                                "today".green()
                            } else if age_days == 1 {
                                "1 day ago".normal()
                            } else if age_days <= 7 {
                                format!("{} days ago", age_days).yellow()
                            } else {
                                format!("{} days ago", age_days).red()
                            };

                            let title = if mention.pr_title.len() > 55 {
                                format!("{}...", &mention.pr_title[..52])
                            } else {
                                mention.pr_title.clone()
                            };

                            println!("{}. {}  #{}  {}{}",
                                i + 1,
                                reason_label,
                                mention.pr_number,
                                title.bold(),
                                unread_marker
                            );
                            println!("   📁 {}  •  ⏱️ {}  •  🔗 {}",
                                mention.repo.dimmed(),
                                age_label,
                                mention.pr_url.blue().underline()
                            );
                            if !mention.last_comment_preview.is_empty() {
                                let preview = if mention.last_comment_preview.len() > 80 {
                                    format!("{}...", &mention.last_comment_preview[..77])
                                } else {
                                    mention.last_comment_preview.clone()
                                };
                                println!("   💬 \"{}\"", preview.dimmed());
                            }
                            if i < total - 1 {
                                println!();
                            }
                        }

                        println!("\n{}", "─".repeat(50));
                        println!("  💡 Use `--unread-only` or `-u` to show only unread notifications");
                        println!("  💡 Use `--json` for scripting\n");
                    }
                }
                Err(e) => {
                    println!("{}", "❌ Failed to fetch notifications".red());
                    println!("   Error: {}", e);
                }
            }
        }

        Commands::Summary { json } => {
            use chrono::Utc;

            if reviews.is_empty() {
                if json {
                    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                        "total": 0,
                        "total_additions": 0,
                        "total_deletions": 0,
                        "oldest_age_days": 0,
                        "draft_count": 0,
                        "by_urgency": { "critical": 0, "high": 0, "medium": 0, "low": 0 },
                        "by_repo": {},
                    }))?);
                } else {
                    println!("✅ No pending reviews. You're all clear!");
                }
                return Ok(());
            }

            let now = Utc::now();
            let total = reviews.len();
            let total_additions: u64 = reviews.iter().map(|r| r.additions).sum();
            let total_deletions: u64 = reviews.iter().map(|r| r.deletions).sum();
            let draft_count = reviews.iter().filter(|r| r.draft).count();

            // Find oldest PR age
            let oldest_age_days = reviews
                .iter()
                .map(|r| (now - r.created_at).num_days())
                .max()
                .unwrap_or(0);

            // Categorize by urgency (based on priority score)
            let mut critical = 0usize; // score 5
            let mut high = 0usize;     // score 4
            let mut medium = 0usize;   // score 3
            let mut low = 0usize;       // score 1-2

            for r in &reviews {
                let score = logger::calculate_priority_score(r);
                match score {
                    5 => critical += 1,
                    4 => high += 1,
                    3 => medium += 1,
                    _ => low += 1,
                }
            }

            // By repo breakdown
            use std::collections::HashMap;
            let mut by_repo: HashMap<String, usize> = HashMap::new();
            for r in &reviews {
                *by_repo.entry(r.repo.clone()).or_insert(0) += 1;
            }

            if json {
                #[derive(serde::Serialize)]
                struct SummaryOutput {
                    total: usize,
                    total_additions: u64,
                    total_deletions: u64,
                    oldest_age_days: i64,
                    draft_count: usize,
                    by_urgency: UrgencyBreakdown,
                    by_repo: HashMap<String, usize>,
                }
                #[derive(serde::Serialize)]
                struct UrgencyBreakdown {
                    critical: usize,
                    high: usize,
                    medium: usize,
                    low: usize,
                }
                let output = SummaryOutput {
                    total,
                    total_additions,
                    total_deletions,
                    oldest_age_days,
                    draft_count,
                    by_urgency: UrgencyBreakdown { critical, high, medium, low },
                    by_repo,
                };
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                // Single-line summary for quick status
                let oldest_label = if oldest_age_days == 0 {
                    "today".green()
                } else if oldest_age_days == 1 {
                    "1d".yellow()
                } else if oldest_age_days <= 3 {
                    format!("{}d", oldest_age_days).yellow()
                } else if oldest_age_days <= 7 {
                    format!("{}d", oldest_age_days).red()
                } else {
                    format!("{}d!!", oldest_age_days).red().bold()
                };

                let urgency_parts: Vec<String> = {
                    let mut parts = Vec::new();
                    if critical > 0 { parts.push(format!("🔥{}/", critical)); }
                    if high > 0 { parts.push(format!("⚡{}/", high)); }
                    if medium > 0 { parts.push(format!("📅{}/", medium)); }
                    if low > 0 { parts.push(format!("💤{}/", low)); }
                    parts
                };
                let urgency_str = if urgency_parts.is_empty() {
                    "".to_string()
                } else {
                    let mut s = urgency_parts.join("");
                    s.pop(); // remove trailing slash
                    format!(" [{}]", s)
                };

                let draft_str = if draft_count > 0 {
                    format!(" ({} draft)", draft_count).yellow().to_string()
                } else {
                    String::new()
                };

                println!(
                    "📋 {} PRs  ⏱️ oldest:{}  +{}/-{} lines{}{}",
                    total.to_string().cyan().bold(),
                    oldest_label,
                    total_additions.to_string().green(),
                    total_deletions.to_string().red(),
                    urgency_str,
                    draft_str
                );

                // Compact repo breakdown
                if !by_repo.is_empty() {
                    let mut repo_parts: Vec<String> = by_repo.iter()
                        .map(|(repo, count)| format!("{}:{}", repo.split('/').last().unwrap_or(repo), count))
                        .collect();
                    repo_parts.sort();
                    println!("   📁 {}", repo_parts.join(" • "));
                }
            }
        }

        Commands::Focus { open, json } => {
            use chrono::Utc;

            if reviews.is_empty() {
                if json {
                    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                        "focused": null,
                        "total_pending": 0,
                        "message": "No pending reviews — you're all clear!"
                    }))?);
                } else {
                    println!("🎉 No pending reviews. You're all clear!");
                    println!("   No PR needs your focus right now.");
                }
                return Ok(());
            }

            // Find the highest-priority PR (by score, then oldest by age)
            let focused = reviews
                .iter()
                .max_by_key(|r| {
                    let score = logger::calculate_priority_score(r);
                    let age_days = (Utc::now() - r.created_at).num_days();
                    (score, age_days)
                })
                .cloned();

            if let Some(pr) = focused {
                let score = logger::calculate_priority_score(&pr);
                let age_days = (Utc::now() - pr.created_at).num_days();
                let total_lines = pr.additions + pr.deletions;

                if json {
                    #[derive(serde::Serialize)]
                    struct FocusOutput<'a> {
                        repo: &'a str,
                        pr_number: u64,
                        pr_title: &'a str,
                        pr_author: &'a str,
                        pr_url: &'a str,
                        score: u8,
                        age_days: i64,
                        additions: u64,
                        deletions: u64,
                        draft: bool,
                    }
                    let output = FocusOutput {
                        repo: &pr.repo,
                        pr_number: pr.pr_number,
                        pr_title: &pr.pr_title,
                        pr_author: &pr.pr_author,
                        pr_url: &pr.pr_url,
                        score,
                        age_days,
                        additions: pr.additions,
                        deletions: pr.deletions,
                        draft: pr.draft,
                    };
                    println!("{}", serde_json::to_string_pretty(&output)?);
                } else {
                    let score_stars = "★".repeat(score as usize).to_string();
                    let age_label = if age_days == 0 {
                        "today".green().to_string()
                    } else if age_days == 1 {
                        "1 day ago".yellow().to_string()
                    } else if age_days <= 3 {
                        format!("{} days ago", age_days).yellow()
                    } else if age_days <= 7 {
                        format!("{} days ago", age_days).red()
                    } else {
                        format!("{} days ago!!", age_days).red().bold().to_string()
                    };

                    let draft_label = if pr.draft { " [DRAFT]".yellow().to_string() } else { String::new() };

                    println!();
                    println!("🎯 YOUR FOCUS PR");
                    println!("{}", "─".repeat(50));
                    println!();
                    println!("  #{}  {}", pr.pr_number, pr.pr_title.bold());
                    println!();
                    println!("  📁 {}  👤 {}  ⏱️ {}  📊 {}/{}",
                        pr.repo.split('/').last().unwrap_or(&pr.repo).dimmed(),
                        pr.pr_author.cyan(),
                        age_label,
                        pr.additions.to_string().green(),
                        pr.deletions.to_string().red()
                    );
                    println!("  📏 Total: {} lines  {}", total_lines, score_stars.red().bold());
                    println!("  🔗 {}", pr.pr_url.blue().underline());
                    println!();
                    println!("{}", "─".repeat(50));
                    println!("  Total pending: {} PRs", reviews.len());
                    if reviews.len() > 1 {
                        println!("  Run `review-dispatcher top` to see more");
                    }
                    println!();

                    if open {
                        opener::open(&pr.pr_url)?;
                        println!("🖥️  Opening PR in browser...");
                    }
                }
            }
        }

        Commands::Conflicts { only_conflicts, json } => {
            println!("\n🔍 Checking merge conflict status for {} PRs...\n", reviews.len());
            io::stdout().flush()?;

            match github::fetch_merge_conflict_status(
                &cfg.github_token,
                &cfg.github_org,
                &reviews,
            )
            .await
            {
                Ok(statuses) => {
                    let conflict_count = statuses.iter().filter(|s| s.has_conflicts).count();
                    let clean_count = statuses.len() - conflict_count;

                    if json {
                        #[derive(serde::Serialize)]
                        struct ConflictOutput<'a> {
                            repo: &'a str,
                            pr_number: u64,
                            pr_title: &'a str,
                            has_conflicts: bool,
                            mergeable: Option<bool>,
                            rebaseable: Option<bool>,
                        }
                        let output: Vec<ConflictOutput> = statuses
                            .iter()
                            .map(|s| ConflictOutput {
                                repo: &s.repo,
                                pr_number: s.pr_number,
                                pr_title: &s.pr_title,
                                has_conflicts: s.has_conflicts,
                                mergeable: s.mergeable,
                                rebaseable: s.rebaseable,
                            })
                            .collect();
                        println!("{}", serde_json::to_string_pretty(&output)?);
                    } else {
                        println!("\n⚠️  Merge Conflict Report\n{}", "─".repeat(50));
                        println!("  ❌ PRs with conflicts: {}", conflict_count.to_string().red().bold());
                        println!("  ✅ Clean PRs:           {}", clean_count.to_string().green().bold());
                        println!("{}", "─".repeat(50));

                        // Sort: conflicts first, then by repo
                        let mut sorted = statuses.clone();
                        sorted.sort_by(|a, b| {
                            let a_conflict = if a.has_conflicts { 0 } else { 1 };
                            let b_conflict = if b.has_conflicts { 0 } else { 1 };
                            if a_conflict != b_conflict {
                                a_conflict.cmp(&b_conflict)
                            } else {
                                a.repo.cmp(&b.repo)
                            }
                        });

                        let mut shown_any = false;
                        for status in sorted {
                            if only_conflicts && !status.has_conflicts {
                                continue;
                            }
                            shown_any = true;

                            if status.has_conflicts {
                                let rebase_label = if status.rebaseable == Some(true) {
                                    " [rebaseable]".yellow()
                                } else {
                                    "".normal()
                                };
                                println!(
                                    "\n  ❌ #{} {} ({}){}",
                                    status.pr_number,
                                    status.pr_title.bold().red(),
                                    status.repo.dimmed(),
                                    rebase_label
                                );
                                println!("      ⚠️  Cannot merge - has merge conflicts");
                                println!("      🔗 {}", format!("{}/pull/{}", status.repo, status.pr_number).blue().underline());
                            } else {
                                println!(
                                    "\n  ✅ #{} {} ({})",
                                    status.pr_number,
                                    status.pr_title.bold().green(),
                                    status.repo.dimmed()
                                );
                            }
                        }

                        if !shown_any {
                            if only_conflicts {
                                println!("\n  🎉 No PRs with conflicts found!\n");
                            } else {
                                println!("\n  No PRs to display.\n");
                            }
                        }

                        println!("{}", "─".repeat(50));
                        if !only_conflicts {
                            println!("\n💡 Use `--only-conflicts` or `-c` to show only PRs with conflicts");
                        }
                        println!();
                    }
                }
                Err(e) => {
                    println!("{}", "❌ Failed to check conflicts".red());
                    println!("   Error: {}", e);
                }
            }
        }

        Commands::Ci { failed_only, passing_only, all, pr_numbers, json } => {
            let targets: Vec<_> = if all {
                if reviews.is_empty() {
                    println!("No pending reviews found.");
                    return Ok(());
                }
                reviews.clone()
            } else if let Some(ref nums) = pr_numbers {
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
                if reviews.is_empty() {
                    println!("No pending reviews found.");
                    return Ok(());
                }
                reviews.clone()
            };

            if targets.is_empty() {
                println!("No PRs to check CI status for.");
                return Ok(());
            }

            println!("\n🔧 Checking CI status for {} PR(s)...\n", targets.len());
            io::stdout().flush()?;

            match github::fetch_ci_status(
                &cfg.github_token,
                &cfg.github_org,
                &targets,
            )
            .await
            {
                Ok(statuses) => {
                    // Apply filters
                    let filtered: Vec<_> = statuses
                        .into_iter()
                        .filter(|s| {
                            if failed_only {
                                s.overall_status == "failure" || s.checks.iter().any(|c| c.conclusion.as_deref() == Some("failure"))
                            } else if passing_only {
                                s.overall_status == "success" || s.checks.iter().all(|c| c.conclusion.as_deref() == Some("success"))
                            } else {
                                true
                            }
                        })
                        .collect();

                    if filtered.is_empty() {
                        if failed_only {
                            println!("🎉 No PRs with failing CI checks!\n");
                        } else if passing_only {
                            println!("❌ No PRs with fully passing CI.\n");
                        } else {
                            println!("No CI status data available.\n");
                        }
                        return Ok(());
                    }

                    let failure_count = filtered.iter().filter(|s| s.overall_status == "failure" || s.checks.iter().any(|c| c.conclusion.as_deref() == Some("failure"))).count();
                    let success_count = filtered.len() - failure_count;

                    if json {
                        println!("{}", serde_json::to_string_pretty(&filtered)?);
                    } else {
                        println!("\n🔧 CI Status Report\n{}", "─".repeat(50));
                        println!("  ❌ Failing:  {}", failure_count.to_string().red().bold());
                        println!("  ✅ Passing:  {}", success_count.to_string().green().bold());
                        println!("{}", "─".repeat(50));

                        // Sort: failures first, then by repo
                        let mut sorted = filtered.clone();
                        sorted.sort_by(|a, b| {
                            let a_fail = if a.overall_status == "failure" || a.checks.iter().any(|c| c.conclusion.as_deref() == Some("failure")) { 0 } else { 1 };
                            let b_fail = if b.overall_status == "failure" || b.checks.iter().any(|c| c.conclusion.as_deref() == Some("failure")) { 0 } else { 1 };
                            if a_fail != b_fail {
                                a_fail.cmp(&b_fail)
                            } else {
                                a.repo.cmp(&b.repo)
                            }
                        });

                        for status in &sorted {
                            let has_failure = status.overall_status == "failure" || status.checks.iter().any(|c| c.conclusion.as_deref() == Some("failure"));
                            let has_success = status.checks.iter().all(|c| c.conclusion.as_deref() == Some("success"));
                            let has_in_progress = status.checks.iter().any(|c| c.status == "in_progress");

                            let status_icon = if has_failure {
                                "❌".red()
                            } else if has_success {
                                "✅".green()
                            } else if has_in_progress {
                                "⏳".yellow()
                            } else {
                                "⚪".normal()
                            };

                            let status_label = if has_failure {
                                "FAILING".red().to_string()
                            } else if has_success {
                                "PASSING".green().to_string()
                            } else if has_in_progress {
                                "IN PROGRESS".yellow().to_string()
                            } else {
                                "UNKNOWN".dimmed().to_string()
                            };

                            println!(
                                "\n{} #{}  {}  ({})\n    Status: {}",
                                status_icon,
                                status.pr_number,
                                status.pr_title.bold(),
                                status.repo.dimmed(),
                                status_label
                            );

                            if status.checks.is_empty() {
                                println!("    (no checks configured)");
                            } else {
                                for check in &status.checks {
                                    let check_status_icon = match check.conclusion.as_deref() {
                                        Some("success") => "✅".to_string(),
                                        Some("failure") => "❌".to_string(),
                                        Some("cancelled") | Some("skipped") => "⚠️ ".to_string(),
                                        Some("timed_out") => "⏱️ ".to_string(),
                                        Some("neutral") => "➖".to_string(),
                                        Some("action_required") => "🔔".to_string(),
                                        None if check.status == "in_progress" => "🔄 ".yellow().to_string(),
                                        None if check.status == "queued" || check.status == "waiting" => "⏳ ".yellow().to_string(),
                                        _ => "⚪ ".to_string(),
                                    };
                                    let check_conclusion = check.conclusion.as_deref().unwrap_or(&check.status);
                                    println!(
                                        "      {}{}  ({})",
                                        check_status_icon,
                                        check.name.dimmed(),
                                        check_conclusion.dimmed()
                                    );
                                }
                            }
                        }

                        println!("{}", "─".repeat(50));
                        println!("\n💡 Use `ci --failed-only` or `-f` to show only failing PRs");
                        println!("💡 Use `ci --passing-only` or `-p` to show only passing PRs\n");
                    }
                }
                Err(e) => {
                    println!("{}", "❌ Failed to check CI status".red());
                    println!("   Error: {}", e);
                }
            }
        }

        Commands::Health { json } => {
            println!("\n🏥 Fetching GitHub API health status...\n");
            io::stdout().flush()?;

            match github::fetch_health_status(&cfg.github_token).await {
                Ok(health) => {
                    if json {
                        println!("{}", serde_json::to_string_pretty(&health)?);
                    } else {
                        println!("🏥 GitHub API Health Status\n{}", "─".repeat(50));

                        // Authentication status
                        if health.authenticated {
                            println!("  ✅ Authenticated as: {}", health.username.as_ref().unwrap_or(&"unknown".to_string()).cyan());
                        } else {
                            println!("  ❌ Not authenticated - check your GITHUB_TOKEN");
                        }
                        println!();

                        // Rate limits
                        for limit in &health.rate_limits {
                            let usage_pct = if limit.limit > 0 {
                                (limit.remaining as f64 / limit.limit as f64) * 100.0
                            } else {
                                100.0
                            };

                            let status_icon = if limit.remaining == 0 {
                                "🔴".red()
                            } else if usage_pct < 10.0 {
                                "🟡".yellow()
                            } else if usage_pct < 50.0 {
                                "🟠".yellow()
                            } else {
                                "🟢".green()
                            };

                            let reset_formatted = if limit.reset_in_seconds > 0 {
                                let hours = limit.reset_in_seconds / 3600;
                                let mins = (limit.reset_in_seconds % 3600) / 60;
                                if hours > 0 {
                                    format!("resets in {}h {}m", hours, mins)
                                } else {
                                    format!("resets in {}m", mins)
                                }
                            } else {
                                "already reset".to_string()
                            };

                            // Visual bar
                            let bar_width = 20;
                            let bar: String = if limit.limit > 0 {
                                format!("{}{}", "█".green(), "░".truecolor(60, 60, 60))
                            } else {
                                "░".repeat(bar_width)
                            };

                            let usage_str = format!("{}/{}", limit.remaining, limit.limit);
                            let reset_str = reset_formatted;

                            println!(
                                "  {} {:12} {} {} ({})",
                                status_icon,
                                limit.resource,
                                bar,
                                usage_str.dimmed(),
                                reset_str.dimmed()
                            );
                        }

                        if health.rate_limit_warning {
                            println!();
                            println!("  ⚠️  {}", "Rate limit warning: API quota is running low!".yellow().bold());
                        }

                        println!("{}", "─".repeat(50));
                        let server_time_str = health.server_time.format("%Y-%m-%d %H:%M:%S UTC").to_string();
                        println!("  🕐 Server time: {}", server_time_str.dimmed());
                        println!();
                        println!("  💡 Use `--json` for scripting");
                        println!("{}", "─".repeat(50));
                    }
                }
                Err(e) => {
                    println!("{}", "❌ Failed to fetch health status".red());
                    println!("   Error: {}", e);
                }
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

fn colorize_label(name: &str, color: &str) -> colored::ColoredString {
    // Parse hex color (GitHub label colors are 6-char hex without #)
    if color.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&color[0..2], 16),
            u8::from_str_radix(&color[2..4], 16),
            u8::from_str_radix(&color[4..6], 16),
        ) {
            // Use the hex color for the text
            let label_with_bg = format!(" {} ", name);
            return label_with_bg.color(colored::Color::TrueColor { r, g, b });
        }
    }
    // Fallback: return name with cyan color
    name.cyan()
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
