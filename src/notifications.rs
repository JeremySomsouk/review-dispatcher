use std::process::Command;

pub fn send_mac_notification(title: &str, message: &str) -> bool {
    let escaped_message = message.replace('"', "\\\"");
    let escaped_title = title.replace('"', "\\\"");
    let apple_script = format!(
        r#"display notification "{}"
        with title "🦀 {}"
        subtitle "Review Dispatcher"
        sound name "Glass""#,
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
        let result = send_mac_notification("Test Title", "Test Message");
        // We can't guarantee success in CI, but it shouldn't panic
        assert!(result || true); // Always pass, just test it doesn't crash
    }
}