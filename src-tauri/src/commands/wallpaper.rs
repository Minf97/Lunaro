use std::process::Command;
use tauri::command;

#[command]
pub fn set_wallpaper_macos(image_path: String) -> Result<String, String> {
    // macos 使用 AppleScript 设置壁纸
    let script = format!(
        r#"
        tell application "System Events"
            tell every desktop
                set picture to "{}"
            end tell
        end tell
        "#,
        image_path
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Wallpaper set successfully".to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to set wallpaper: {}", stderr))
    }
}

#[command]
pub fn get_wallpaper_dir() -> Result<String, String> {
    let home_dir = std::env::var("HOME").map_err(|e| e.to_string())?;
    let wallpaper_dir = format!("{}/Pictures/Lunaro", home_dir);

    // 创建目录
    std::fs::create_dir_all(&wallpaper_dir).map_err(|e| e.to_string())?;

    Ok(wallpaper_dir)
}
