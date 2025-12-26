use super::errors::Error;
use std::fs;
use std::process::Command;

#[tauri::command]
pub fn read(path: String) -> Result<String, Error> {
    let data = fs::read(path)?;
    let string = String::from_utf8(data)?;
    Ok(string)
}

#[tauri::command]
pub fn write(path: String, contents: String) -> Result<(), Error> {
    fs::write(path, contents)?;
    Ok(())
}

#[tauri::command]
pub fn write_binary(path: String, contents: Vec<u8>) -> Result<(), Error> {
    fs::write(path, contents)?;
    Ok(())
}

#[tauri::command]
pub fn open_folder(path: String) -> Result<(), Error> {
    #[cfg(target_os = "macos")]
    Command::new("open").arg(&path).spawn()?;

    #[cfg(target_os = "windows")]
    Command::new("explorer").arg(&path).spawn()?;

    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(&path).spawn()?;

    Ok(())
}

#[tauri::command]
pub fn delete_folder_contents(path: String) -> Result<String, Error> {
    let entries = fs::read_dir(&path)?;
    let mut count = 0;

    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_file() {
            fs::remove_file(file_path)?;
            count += 1;
        }
    }

    Ok(format!("Deleted {} files", count))
}
