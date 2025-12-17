mod commands;
use commands::default::{read, write, write_binary};
use commands::shortcuts;
use commands::wallpaper::{get_wallpaper_dir, set_wallpaper_macos};
use tauri::Manager;

#[allow(clippy::missing_panics_doc)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // 注册全局快捷键
            shortcuts::register_shortcuts(&app.handle())?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            read,
            write,
            write_binary,
            set_wallpaper_macos,
            get_wallpaper_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
