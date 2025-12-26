use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

/// 注册所有全局快捷键
pub fn register_shortcuts(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    register_settings_shortcut(app)?;
    Ok(())
}

/// 注册设置快捷键 (Command+Shift+S/Ctrl+Shift+S)
fn register_settings_shortcut(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.clone();

    // 根据平台选择快捷键
    let shortcut = if cfg!(target_os = "macos") {
        "Command+Shift+S"
    } else {
        "Ctrl+Shift+S"
    };

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, _event| {
            let _ = handle.emit("settings:open", ());
        })?;

    Ok(())
}
