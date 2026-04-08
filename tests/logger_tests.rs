//! Unit tests for logger / priority scoring.

use prctrl::logger::calculate_priority_score_for_stats;

#[test]
fn test_priority_score_small_new_pr() {
    // Small PR (<50 lines), opened today → score = 1
    let score = calculate_priority_score_for_stats(30, 0);
    assert_eq!(score, 1);
}

#[test]
fn test_priority_score_zero_days() {
    // 0 days old, small → score = 1 (clamped minimum)
    let score = calculate_priority_score_for_stats(30, 0);
    assert_eq!(score, 1);
}

#[test]
fn test_priority_score_medium_age_medium_size() {
    // 7 days old (age_score=1.0), 100 lines (size_score=1.0)
    // combined = (1.0*0.6 + 1.0*0.4)/7.0*5.0 ≈ 0.71, clamped to 1
    let score = calculate_priority_score_for_stats(100, 7);
    assert_eq!(score, 1);
}

#[test]
fn test_priority_score_large_old_pr() {
    // 45 days old (age_score=4.0), 600 lines (size_score=3.0)
    // combined = (4.0*0.6 + 3.0*0.4)/7.0*5.0 ≈ 2.57, clamped to range [1,5]
    let score = calculate_priority_score_for_stats(600, 45);
    assert!((2..=4).contains(&score));
}

#[test]
fn test_priority_score_old_medium() {
    // 30 days old (age_score=3.0), 300 lines (size_score=2.0)
    // combined = (3.0*0.6 + 2.0*0.4)/7.0*5.0 ≈ 1.86, clamped to 1
    let score = calculate_priority_score_for_stats(300, 30);
    assert!((1..=3).contains(&score));
}

#[test]
fn test_priority_score_boundary_50_lines() {
    // Exactly 50 lines (boundary between small and medium)
    let score = calculate_priority_score_for_stats(50, 5);
    assert!((1..=3).contains(&score));
}
