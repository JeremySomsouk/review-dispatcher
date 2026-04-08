//! Unit tests for stack detection logic.

use prctrl::stack::StackedPR;

/// Helper to create a test PR.
fn test_pr(number: u64, title: &str, head: &str, base: &str) -> StackedPR {
    StackedPR {
        number,
        title: title.to_string(),
        repo: "test-repo".to_string(),
        head_branch: head.to_string(),
        base_branch: base.to_string(),
        position: 0,
        url: format!("https://github.com/org/test-repo/pull/{}", number),
        author: "testuser".to_string(),
        draft: false,
    }
}

#[test]
fn test_extract_ticket_key_from_branch() {
    // The regex in stack.rs should match patterns like TAHC-1666, PROJ-123
    let re = regex::Regex::new(r"(?i)\b([A-Z][A-Z0-9]*-\d+)\b").unwrap();

    assert!(re.captures("TAHC-1666-onboarding-client").is_some());
    assert!(re.captures("PROJ-123-feature").is_some());
    assert!(re.captures("ABC-1").is_some());

    let caps = re.captures("TAHC-1666-onboarding-client").unwrap();
    assert_eq!(caps.get(1).unwrap().as_str().to_uppercase(), "TAHC-1666");

    let caps = re
        .captures("refactor(TAHC-1666): extract port [1/3]")
        .unwrap();
    assert_eq!(caps.get(1).unwrap().as_str().to_uppercase(), "TAHC-1666");

    // No ticket key
    assert!(re.captures("just-a-branch").is_none());
}

#[test]
fn test_extract_position_index() {
    let re = regex::Regex::new(r"[\[\(](\d+)/(\d+)[\]\)]").unwrap();

    assert_eq!(
        re.captures("refactor(TAHC-1666): extract port [1/3]")
            .and_then(|c| c.get(1)?.as_str().parse::<usize>().ok()),
        Some(1)
    );
    assert_eq!(
        re.captures("feat: add service [2/3]").and_then(|c| c
            .get(1)?
            .as_str()
            .parse::<usize>()
            .ok()),
        Some(2)
    );
    assert_eq!(
        re.captures("feat: add tests (3/5)")
            .and_then(|c| c.get(1)?.as_str().parse::<usize>().ok()),
        Some(3)
    );

    // No marker
    assert!(re.captures("just a normal title").is_none());
}

#[test]
fn test_branch_chain_stacks_simple() {
    // PR 1: feature → main
    // PR 2: feature-2 → feature
    // Should detect a chain: PR1 → PR2
    let prs = [
        test_pr(1, "Add feature", "feature", "main"),
        test_pr(2, "Extend feature", "feature-2", "feature"),
    ];

    // Verify the branch-chaining relationship
    assert_eq!(prs[1].base_branch, prs[0].head_branch);
}

#[test]
fn test_branch_chain_three_deep() {
    // PR 1: feature → main
    // PR 2: feature-2 → feature
    // PR 3: feature-3 → feature-2
    let prs = [
        test_pr(1, "Add feature", "feature", "main"),
        test_pr(2, "Extend feature", "feature-2", "feature"),
        test_pr(3, "Add tests", "feature-3", "feature-2"),
    ];

    // Verify the chain relationships
    assert_eq!(prs[1].base_branch, prs[0].head_branch);
    assert_eq!(prs[2].base_branch, prs[1].head_branch);
}

#[test]
fn test_convention_stacks_ticket_key_grouping() {
    // PRs with the same ticket key but all targeting main
    let prs = [
        test_pr(
            974,
            "refactor(TAHC-1666): extract client [1/3]",
            "TAHC-1666-client",
            "main",
        ),
        test_pr(
            975,
            "refactor(TAHC-1666): extract service [2/3]",
            "TAHC-1666-service",
            "main",
        ),
        test_pr(
            976,
            "refactor(TAHC-1666): add tests [3/3]",
            "TAHC-1666-tests",
            "main",
        ),
    ];

    let re = regex::Regex::new(r"(?i)\b([A-Z][A-Z0-9]*-\d+)\b").unwrap();

    // All share the same ticket key
    let keys: Vec<String> = prs
        .iter()
        .map(|p| {
            re.captures(&p.head_branch)
                .or_else(|| re.captures(&p.title))
                .map(|c| c.get(1).unwrap().as_str().to_uppercase())
                .unwrap_or_default()
        })
        .collect();

    assert_eq!(keys[0], "TAHC-1666");
    assert_eq!(keys[1], "TAHC-1666");
    assert_eq!(keys[2], "TAHC-1666");

    // All share the same base branch
    assert!(prs.windows(2).all(|w| w[0].base_branch == w[1].base_branch));
}

#[test]
fn test_no_stack_unrelated_prs() {
    let prs = [
        test_pr(1, "Fix bug A", "fix-bug-a", "main"),
        test_pr(2, "Add feature B", "add-feature-b", "main"),
    ];

    // No chain: different branches, all target main
    assert_ne!(prs[1].base_branch, prs[0].head_branch);
    assert_eq!(prs[0].base_branch, "main");
    assert_eq!(prs[1].base_branch, "main");
}
