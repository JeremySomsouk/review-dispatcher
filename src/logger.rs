use crate::github::PendingReview;
use chrono::Utc;
use colored::*;

/// Calculate priority score (1-5) based on age and size
pub fn calculate_priority_score(review: &PendingReview) -> u8 {
    let age_days = (Utc::now() - review.created_at).num_days() as f64;
    let size = review.additions + review.deletions;
    
    // Age score: 0-3 days = 0, 3-7 = 1, 7-14 = 2, 14-30 = 3, 30+ = 4
    let age_score = if age_days <= 3.0 {
        0.0
    } else if age_days <= 7.0 {
        1.0
    } else if age_days <= 14.0 {
        2.0
    } else if age_days <= 30.0 {
        3.0
    } else {
        4.0
    };
    
    // Size score: small (<50) = 0, medium (50-200) = 1, large (200-500) = 2, huge (>500) = 3
    let size_score = if size < 50 {
        0.0
    } else if size < 200 {
        1.0
    } else if size < 500 {
        2.0
    } else {
        3.0
    };
    
    let combined: f64 = (age_score * 0.6 + size_score * 0.4) / 7.0 * 5.0;
    
    // Map to 1-5 scale
    let score = (combined.clamp(1.0, 5.0) as u8).max(1).min(5);
    score
}

/// Show priority stars (e.g., "⭐⭐⭐")
pub fn priority_stars(score: u8) -> String {
    let stars = score.min(5) as usize;
    "⭐".repeat(stars)
}

/// Calculate priority score (1-5) based on size and age (for stats/trends)
pub fn calculate_priority_score_for_stats(size: u64, age_days: u32) -> u8 {
    // Age score: 0-3 days = 0, 3-7 = 1, 7-14 = 2, 14-30 = 3, 30+ = 4
    let age_score = if age_days <= 3 {
        0.0
    } else if age_days <= 7 {
        1.0
    } else if age_days <= 14 {
        2.0
    } else if age_days <= 30 {
        3.0
    } else {
        4.0
    };

    // Size score: small (<50) = 0, medium (50-200) = 1, large (200-500) = 2, huge (>500) = 3
    let size_score = if size < 50 {
        0.0
    } else if size < 200 {
        1.0
    } else if size < 500 {
        2.0
    } else {
        3.0
    };

    let combined: f64 = (age_score * 0.6 + size_score * 0.4) / 7.0 * 5.0;

    // Map to 1-5 scale
    let score = (combined.clamp(1.0, 5.0) as u8).max(1).min(5);
    score
}

pub fn print_reviews(reviews: &[PendingReview], show_priority: bool) {
    if reviews.is_empty() {
        println!(
            "{}",
            "✅  No pending reviews. You're all clear."
                .green()
                .bold()
        );
        return;
    }

    println!(
        "\n{}\n",
        format!("🔍  {} pending review(s) assigned to you", reviews.len())
            .yellow()
            .bold()
    );

    for (i, r) in reviews.iter().enumerate() {
        let age_days = (Utc::now() - r.created_at).num_days();
        let age_label = match age_days {
            0 => "today".green(),
            1 => "yesterday".normal(),
            2..=3 => format!("{} days ago", age_days).normal(),
            _ => format!("{} days ago", age_days).red(),
        };

        let draft_label = if r.draft { " [DRAFT]".yellow() } else { "".normal() };
        
        let priority_label = if show_priority {
            let score = calculate_priority_score(r);
            format!(" {}", priority_stars(score).dimmed())
        } else {
            String::new()
        };

        println!(
            "  {}  {} {} {}{}{}",
            format!("[{}]", i + 1).dimmed(),
            r.pr_title.bold(),
            format!("#{}", r.pr_number).dimmed(),
            format!("({})", r.repo).dimmed(),
            draft_label,
            priority_label
        );
        println!(
            "      👤 {}  •  +{} -{} lines  •  opened {}",
            r.pr_author.cyan(),
            r.additions.to_string().green(),
            r.deletions.to_string().red(),
            age_label
        );
        println!("      🌿 {}  🔗 {}", r.branch.dimmed(), r.pr_url.underline().blue());
        println!();
    }
}

pub fn print_delegate_result(review: &PendingReview, summary: &str) {
    println!("{}", "─".repeat(60).dimmed());
    println!(
        "{}  {} {}",
        "🤖 Claude on:".purple().bold(),
        review.pr_title.bold(),
        format!("#{}", review.pr_number).dimmed()
    );
    println!("{}", summary);
    println!("🔗 {}", review.pr_url.underline().blue());
    println!("{}\n", "─".repeat(60).dimmed());
}
