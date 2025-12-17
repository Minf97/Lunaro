use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

/// 注册所有全局快捷键
pub fn register_shortcuts(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    register_refresh_shortcut(app)?;
    Ok(())
}

/// 注册刷新快捷键 (Command+R/Ctrl+R)
fn register_refresh_shortcut(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.clone();

    // 根据平台选择快捷键
    let shortcut = if cfg!(target_os = "macos") {
        "Command+R"
    } else {
        "Ctrl+R"
    };

    app.global_shortcut().on_shortcut(shortcut, move |app, _shortcut, _event| {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.eval("window.location.reload()");
        }
    })?;

    Ok(())
}
