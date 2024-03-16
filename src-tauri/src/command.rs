use std::fs;

use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn close_splashscreen(app: AppHandle) {
	println!("close_splashscreen");
	// Close splashscreen
	if let Some(splashscreen) = app.get_webview_window("splashscreen") {
		splashscreen.close().unwrap();
	}
	// Show main window
	app.get_webview_window("main").unwrap().show().unwrap();
}

#[derive(Clone, serde::Serialize)]
struct FileName {
	name: String
}

#[tauri::command]
pub fn get_files_from_base_dir(app: AppHandle) -> Result<Vec<String>, bool> {
	let app_config_dir = app.path().app_config_dir().unwrap();
	let app_cache_dir = app.path().app_cache_dir().unwrap();
	let app_data_dir = app.path().app_data_dir().unwrap();
	let app_log_dir = app.path().app_log_dir().unwrap();
	let app_local_data_dir = app.path().app_local_data_dir().unwrap();
	// let font_dir = app.path().font_dir().unwrap();
	let home_dir = app.path().home_dir().unwrap();
	let cache_dir = app.path().cache_dir().unwrap();
	let data_dir = app.path().data_dir().unwrap();
	let config_dir = app.path().config_dir().unwrap();
	let local_data_dir = app.path().local_data_dir().unwrap();
	println!("app_config_dir = {:?}", app_config_dir);
	println!("app_cache_dir = {:?}", app_cache_dir);
	println!("app_data_dir = {:?}", app_data_dir);
	println!("app_log_dir = {:?}", app_log_dir);
	println!("app_local_data_dir = {:?}", app_local_data_dir);
	// println!("font_dir = {:?}", font_dir);
	println!("home_dir = {:?}", home_dir);
	println!("cache_dir = {:?}", cache_dir);
	println!("data_dir = {:?}", data_dir);
	println!("config_dir = {:?}", config_dir);
	println!("local_data_dir = {:?}", local_data_dir);
	let mut config_path = app_data_dir.clone();
	config_path.push("configs");

	let mut config_file_names: Vec<String> = vec![];
	if let Ok(entries) = fs::read_dir(config_path) {
		for entry in entries.flatten() {
			let file_path = entry.path();
			if let Some(extension) = file_path.extension() {
					if extension == "json" {
						config_file_names.push(file_path.display().to_string());
					}
			}
		}
	}
	Ok(config_file_names)
}
