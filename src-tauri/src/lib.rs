use commandes::{get_file_count};
use errors::SurimiError;
use log::info;

mod commandes;
mod errors;
mod tests;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run()-> Result<(), tauri::Error> {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())

        .setup(|_app| {
            get_os().map_err(|e| e.into())
        })

        .invoke_handler(tauri::generate_handler![
            get_file_count
        ])

        .run(tauri::generate_context!())
        
}

fn get_os() -> Result<(), SurimiError> {
    let platform = tauri_plugin_os::platform();
    info!("Platform: {}", platform);

    // Si aucune erreur n'est possible, retourne simplement Ok(())
    Ok(())
}