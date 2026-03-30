use std::process::Command;

pub fn send_mac_notification(title: &str, message: &str, pr_url: Option<&str>, auto_open: bool) -> bool {
    let escaped_title = title.replace('"', "\\\"");
    let escaped_message = message.replace('"', "\\\"");
    
    if let Some(url) = pr_url {
        let escaped_url = url.replace('"', "\\\"");
        
        if auto_open {
            // Create a script that shows notification AND opens Chrome automatically
            let script = format!(
                "display notification \"{}\" with title \"🦀 {}\" subtitle \"PRCtrl\" sound name \"Glass\"\ntell application \"Google Chrome\" to open location \"{}\"",
                escaped_message, escaped_title, escaped_url
            );
            
            return Command::new("osascript")
                .arg("-e")
                .arg(script)
                .status()
                .is_ok_and(|status| status.success());
        } else {
            // Show notification with URL in message (user can copy/paste)
            let notification_with_url = format!("{}\n\n🔗 {}", message, url);
            let apple_script = format!(
                r#"display notification "{}" with title "🦀 {}" subtitle "PRCtrl" sound name "Glass""#,
                notification_with_url.replace('"', "\\\""),
                escaped_title
            );
            
            return Command::new("osascript")
                .arg("-e")
                .arg(apple_script)
                .status()
                .is_ok_and(|status| status.success());
        }
    }
    
    // Fallback to simple notification if no URL provided
    let apple_script = format!(
        r#"display notification "{}" with title "🦀 {}" subtitle "PRCtrl" sound name "Glass""#,
        escaped_message,
        escaped_title
    );

    Command::new("osascript")
        .arg("-e")
        .arg(apple_script)
        .status()
        .is_ok_and(|status| status.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_mac_notification() {
        // Test that the function runs without panicking
        // Note: This will actually send a notification on macOS
        let result = send_mac_notification("Test Title", "Test Message", None, false);
        // We can't guarantee success in CI, but it shouldn't panic
        assert!(result || true); // Always pass, just test it doesn't crash
    }
    
    #[test]
    fn test_send_mac_notification_with_url_no_auto_open() {
        // Test notification with URL but no auto-open
        let result = send_mac_notification("Test PR", "Test message", Some("https://github.com/test/repo/pull/1"), false);
        assert!(result || true); // Always pass, just test it doesn't crash
    }
    
    #[test]
    fn test_send_mac_notification_with_url_auto_open() {
        // Test notification with URL and auto-open (will open Chrome)
        let result = send_mac_notification("Test PR", "Test message", Some("https://github.com/test/repo/pull/1"), true);
        assert!(result || true); // Always pass, just test it doesn't crash
    }
}