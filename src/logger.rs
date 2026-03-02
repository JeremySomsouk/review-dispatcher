use crate::github::PendingReview;
use chrono::Utc;
use colored::*;

pub fn print_reviews(reviews: &[PendingReview]) {
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

        println!(
            "  {}  {} {} {}{}",
            format!("[{}]", i + 1).dimmed(),
            r.pr_title.bold(),
            format!("#{}", r.pr_number).dimmed(),
            format!("({})", r.repo).dimmed(),
            draft_label
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
