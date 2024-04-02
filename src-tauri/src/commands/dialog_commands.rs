use log::info;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn pick_app_image(app: AppHandle) -> Result<String, String> {
    info!("Opening file dialog");
    let file_path = app
        .dialog()
        .file()
        .add_filter("AppImages", &["AppImage"])
        .blocking_pick_file();

    match file_path {
        Some(path) => {
            println!("Selected file: {:?}", path.path.to_string_lossy());
            Ok(path
                .path
                .into_os_string()
                .into_string()
                .unwrap_or_else(|_| "Invalid path".to_string()))
        }
        None => Err("No file selected".to_string()),
    }
}

#[tauri::command]
pub async fn pick_app_icon(app: AppHandle) -> Result<String, String> {
    info!("Opening file dialog");
    let file_path = app
        .dialog()
        .file()
        .add_filter("Icon", &["png", "jpeg", "jpg"])
        .blocking_pick_file();

    match file_path {
        Some(path) => {
            println!("Selected file: {:?}", path.path.to_string_lossy());
            Ok(path
                .path
                .into_os_string()
                .into_string()
                .unwrap_or_else(|_| "Invalid path".to_string()))
        }
        None => Err("No file selected".to_string()),
    }
}

#[tauri::command]
pub async fn pick_dir(app: AppHandle) -> Result<String, String> {
    info!("Opening directory dialog");
    let dir_path = app
        .dialog()
        .file()
        .add_filter("Icon", &["png", "jpeg", "jpg"])
        .blocking_pick_folder();

    match dir_path {
        Some(path) => {
            println!("Selected directory: {:?}", path.to_string_lossy());
            Ok(path
                .into_os_string()
                .into_string()
                .unwrap_or_else(|_| "Invalid path".to_string()))
        }
        None => Err("No directory selected".to_string()),
    }
}
