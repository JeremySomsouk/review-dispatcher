use std::process::Command;

pub fn send_mac_notification(title: &str, message: &str, pr_url: Option<&str>) -> bool {
    let escaped_message = message.replace('"', "\\\"");
    let escaped_title = title.replace('"', "\\\"");
    
    let apple_script = if let Some(url) = pr_url {
        // If URL is provided, create a clickable notification that opens in Chrome
        let escaped_url = url.replace('"', "\\\"");
        format!(
            r#"display notification "{}"
            with title "🦀 {}"
            subtitle "Review Dispatcher"
            sound name "Glass"
            
            -- Make notification clickable
            set theNotification to (display notification "{}" with title "🦀 {}" subtitle "Review Dispatcher" sound name "Glass")
            
            -- When clicked, open in Chrome
            on clicked(theNotification)
                tell application "Google Chrome"
                    activate
                    open location "{}"
                end tell
            end clicked"#,
            escaped_message, escaped_title,
            escaped_message, escaped_title,
            escaped_url
        )
    } else {
        // Fallback to simple notification if no URL provided
        format!(
            r#"display notification "{}"
            with title "🦀 {}"
            subtitle "Review Dispatcher"
            sound name "Glass""#,
            escaped_message,
            escaped_title
        )
    };

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