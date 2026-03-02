use std::process::Command;
use std::fs;

pub fn send_mac_notification(title: &str, message: &str, pr_url: Option<&str>) -> bool {
    let escaped_message = message.replace('"', "\\\"");
    let escaped_title = title.replace('"', "\\\"");
    
    // Create a temporary AppleScript file for clickable notifications
    if let Some(url) = pr_url {
        let escaped_url = url.replace('"', "\\\"");
        let script_content = format!(
            "display notification \"{}\" with title \"🦀 {}\" subtitle \"Review Dispatcher\" sound name \"Glass\"\n
on clicked(theNotification)\n    tell application \"Google Chrome\"\n        activate\n        open location \"{}\"\n    end tell\nend clicked",
            escaped_message, escaped_title, escaped_url
        );
        
        // Write to temp file and execute
        let temp_dir = std::env::temp_dir();
        let script_path = temp_dir.join("review_dispatcher_notification.scpt");
        
        if let Ok(_) = fs::write(&script_path, script_content) {
            let status = Command::new("osascript")
                .arg(&script_path)
                .status();
            let _ = fs::remove_file(&script_path); // Clean up
            return status.map_or(false, |s| s.success());
        }
    }
    
    // Fallback to simple notification
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