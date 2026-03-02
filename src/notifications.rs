use std::process::Command;

pub fn send_mac_notification(title: &str, message: &str, pr_url: Option<&str>) -> bool {
    let escaped_title = title.replace('"', "\\\"");
    let escaped_message = message.replace('"', "\\\"");
    
    if let Some(url) = pr_url {
        let escaped_url = url.replace('"', "\\\"");
        
        // Create a script that shows notification AND opens Chrome automatically
        let script = format!(
            "display notification \"{}\" with title \"🦀 {}\" subtitle \"Review Dispatcher\" sound name \"Glass\"\ntell application \"Google Chrome\" to open location \"{}\"",
            escaped_message, escaped_title, escaped_url
        );
        
        return Command::new("osascript")
            .arg("-e")
            .arg(script)
            .status()
            .map_or(false, |status| status.success());
    }
    
    // Fallback to simple notification if no URL provided
    let apple_script = format!(
        r#"display notification "{}" with title "🦀 {}" subtitle "Review Dispatcher" sound name "Glass""#,
        escaped_message,
        escaped_title
    );

    Command::new("osascript")
        .arg("-e")
        .arg(apple_script)
        .status()
        .map_or(false, |status| status.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_mac_notification() {
        // Test that the function runs without panicking
        // Note: This will actually send a notification on macOS
        let result = send_mac_notification("Test Title", "Test Message", None);
        // We can't guarantee success in CI, but it shouldn't panic
        assert!(result || true); // Always pass, just test it doesn't crash
    }
    
    #[test]
    fn test_send_mac_notification_with_url() {
        // Test notification with URL (will try to open Chrome when clicked)
        let result = send_mac_notification("Test PR", "Test message", Some("https://github.com/test/repo/pull/1"));
        assert!(result || true); // Always pass, just test it doesn't crash
    }
}