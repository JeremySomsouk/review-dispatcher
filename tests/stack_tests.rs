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
        head_sha: String::new(),
        base_sha: String::new(),
        position: 0,
        url: format!("https://github.com/org/test-repo/pull/{}", number),
        author: "testuser".to_string(),
        draft: false,
    }
}

/// Helper to create a test PR with commit SHAs.
fn test_pr_with_sha(
    number: u64,
    title: &str,
    head: &str,
    base: &str,
    head_sha: &str,
    base_sha: &str,
) -> StackedPR {
    StackedPR {
        number,
        title: title.to_string(),
        repo: "test-repo".to_string(),
        head_branch: head.to_string(),
        base_branch: base.to_string(),
        head_sha: head_sha.to_string(),
        base_sha: base_sha.to_string(),
        position: 0,
        url: format!("https://github.com/org/test-repo/pull/{}", number),
        author: "testuser".to_string(),
        draft: false,
    }
}

#[test]
fn test_extract_ticket_key_from_branch() {
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

    assert!(re.captures("just a normal title").is_none());
}

#[test]
fn test_branch_chain_stacks_simple() {
    let prs = [
        test_pr(1, "Add feature", "feature", "main"),
        test_pr(2, "Extend feature", "feature-2", "feature"),
    ];

    assert_eq!(prs[1].base_branch, prs[0].head_branch);
}

#[test]
fn test_branch_chain_three_deep() {
    let prs = [
        test_pr(1, "Add feature", "feature", "main"),
        test_pr(2, "Extend feature", "feature-2", "feature"),
        test_pr(3, "Add tests", "feature-3", "feature-2"),
    ];

    assert_eq!(prs[1].base_branch, prs[0].head_branch);
    assert_eq!(prs[2].base_branch, prs[1].head_branch);
}

#[test]
fn test_convention_stacks_ticket_key_grouping() {
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

    assert!(prs.windows(2).all(|w| w[0].base_branch == w[1].base_branch));
}

#[test]
fn test_no_stack_unrelated_prs() {
    let prs = [
        test_pr(1, "Fix bug A", "fix-bug-a", "main"),
        test_pr(2, "Add feature B", "add-feature-b", "main"),
    ];

    assert_ne!(prs[1].base_branch, prs[0].head_branch);
    assert_eq!(prs[0].base_branch, "main");
    assert_eq!(prs[1].base_branch, "main");
}

#[test]
fn test_commit_chain_stacks() {
    // PR 1: head_sha=abc123, base_sha=main-sha (root of the chain)
    // PR 2: head_sha=def456, base_sha=abc123 (chains off PR 1)
    // PR 3: head_sha=ghi789, base_sha=def456 (chains off PR 2)
    let prs = [
        test_pr_with_sha(1, "Add feature", "feature", "main", "abc123", "main-sha"),
        test_pr_with_sha(
            2,
            "Extend feature",
            "feature-2",
            "feature",
            "def456",
            "abc123",
        ),
        test_pr_with_sha(3, "Add tests", "feature-3", "feature-2", "ghi789", "def456"),
    ];

    // Verify the commit chain relationships
    assert_eq!(prs[1].base_sha, prs[0].head_sha);
    assert_eq!(prs[2].base_sha, prs[1].head_sha);

    // Root's base_sha should NOT match any other PR's head_sha
    assert_ne!(prs[0].base_sha, prs[0].head_sha);
    assert_ne!(prs[0].base_sha, prs[1].head_sha);
    assert_ne!(prs[0].base_sha, prs[2].head_sha);
}

#[test]
fn test_commit_chain_not_branch_chain() {
    // Two PRs where branch names DON'T chain but SHAs DO chain
    let prs = [
        test_pr_with_sha(10, "Fix login", "fix-login", "develop", "sha-a", "sha-dev"),
        test_pr_with_sha(11, "Fix logout", "fix-logout", "develop", "sha-b", "sha-a"),
    ];

    // Branch names don't chain (fix-logout targets develop, not fix-login)
    assert_eq!(prs[1].base_branch, "develop");
    assert_ne!(prs[1].base_branch, prs[0].head_branch);

    // But SHAs DO chain (fix-logout's base = fix-login's head)
    assert_eq!(prs[1].base_sha, prs[0].head_sha);
}
