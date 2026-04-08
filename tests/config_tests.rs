//! Unit tests for config loading.

#[test]
fn test_config_toml_parsing() {
    // Verify TOML table parsing for repos
    let toml_str = r#"
[github]
token = "ghp_test123"
username = "testuser"
org = "testorg"
repos = ["repo1", "repo2", "repo3"]
teams = ["team1", "team2"]
"#;
    let parsed: toml::Value = toml::from_str(toml_str).unwrap();
    let github = parsed.get("github").unwrap().as_table().unwrap();
    let repos = github.get("repos").unwrap().as_array().unwrap();
    assert_eq!(repos.len(), 3);
    assert_eq!(repos[0].as_str().unwrap(), "repo1");
}

#[test]
fn test_config_toml_empty() {
    let toml_str = "";
    let parsed: toml::Value = toml::from_str(toml_str).unwrap();
    assert!(parsed.as_table().unwrap().is_empty());
}

#[test]
fn test_config_toml_crew_members() {
    let toml_str = r#"
[github]
token = "ghp_test"
username = "user"
org = "org"
crew_members = ["alice", "bob", "carol"]
"#;
    let parsed: toml::Value = toml::from_str(toml_str).unwrap();
    let github = parsed.get("github").unwrap().as_table().unwrap();
    let crew = github.get("crew_members").unwrap().as_array().unwrap();
    assert_eq!(crew.len(), 3);
}
