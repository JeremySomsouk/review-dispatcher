use std::path::Path;

use anyhow::Result;

/// Open a new terminal tab/window at `dir`.
/// Supports macOS (iTerm2 + Terminal.app fallback) and Linux (gnome-terminal + xterm fallback).
pub fn open_terminal_at(dir: &Path) -> Result<()> {
    let dir_str = dir.to_string_lossy();

    #[cfg(target_os = "macos")]
    {
        let iterm_script = format!(
            r#"tell application "iTerm2"
                 tell current window
                   create tab with default profile
                   tell current session
                     write text "cd {dir} && clear && ls"
                   end tell
                 end tell
               end tell"#,
            dir = dir_str
        );

        let result = std::process::Command::new("osascript")
            .arg("-e")
            .arg(&iterm_script)
            .status();

        if result.is_err() || !result.unwrap().success() {
            // Fallback to Terminal.app
            let terminal_script = format!(
                r#"tell application "Terminal"
                     do script "cd {dir} && clear && ls"
                     activate
                   end tell"#,
                dir = dir_str
            );
            std::process::Command::new("osascript")
                .arg("-e")
                .arg(&terminal_script)
                .status()?;
        }
    }

    #[cfg(target_os = "linux")]
    {
        let result = std::process::Command::new("gnome-terminal")
            .args(["--working-directory", &dir_str])
            .spawn();

        if result.is_err() {
            std::process::Command::new("xterm")
                .args(["-e", &format!("cd {} && bash", dir_str)])
                .spawn()?;
        }
    }

    Ok(())
}
