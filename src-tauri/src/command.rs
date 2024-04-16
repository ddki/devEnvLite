use std::{collections::HashSet, fs};

use tauri::{AppHandle, Manager, Runtime};

use crate::{
	environment_vars::{get_environment_vars_manager, EnvironmentVars, EnvironmentVarsType},
	error::AppError,
};

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

#[tauri::command]
pub fn get_config_ids(app: AppHandle) -> Result<Vec<String>, AppError> {
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
	config_path.push("config");

	let mut config_ids: Vec<String> = vec![];
	if let Ok(entries) = fs::read_dir(config_path) {
		for entry in entries.flatten() {
			let file_path = entry.path();
			if let Some(extension) = file_path.extension() {
				println!("extension: {:?}", extension);
				println!("file_name: {:?}", file_path.file_name());
				if extension == "json" {
					let config_file_name =
						file_path.file_name().unwrap().to_str().unwrap().to_string();
					let id = str::replace(&config_file_name, ".json", "");
					config_ids.push(id);
				}
			}
		}
	}
	Ok(config_ids)
}

#[tauri::command]
pub async fn get_keys<R: Runtime>(
	scopes: Vec<String>,
	_app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<Vec<String>, AppError> {
	println!("get_keys: scopes: {:?}", scopes);
	let mut keys = HashSet::new();
	if scopes.contains(&EnvironmentVarsType::SYSTEM.to_string()) {
		let manager = get_environment_vars_manager(&EnvironmentVarsType::SYSTEM);
		keys.extend(manager.inner().get_keys().unwrap());
	}
	if scopes.contains(&EnvironmentVarsType::USER.to_string()) {
		let manager = get_environment_vars_manager(&EnvironmentVarsType::USER);
		keys.extend(manager.inner().get_keys().unwrap());
	}
	let mut sort_keys: Vec<String> = keys.iter().cloned().collect();
	sort_keys.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
	println!("get_keys: keys: {:?}", sort_keys);
	Ok(sort_keys)
}

/// collate environment variables
#[tauri::command]
pub async fn collate_envs<R: Runtime>(
	keys: Vec<String>,
	scopes: Vec<String>,
	_app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	println!("collate_envs: keys: {:?}, scopes: {:?}", keys, scopes);
	if scopes.contains(&EnvironmentVarsType::SYSTEM.to_string()) {
		let manager = get_environment_vars_manager(&EnvironmentVarsType::SYSTEM);
		manager.inner().collate(keys.clone())?;
	}
	if scopes.contains(&EnvironmentVarsType::USER.to_string()) {
		let manager = get_environment_vars_manager(&EnvironmentVarsType::USER);
		manager.inner().collate(keys.clone())?;
	}
	Ok(())
}

/// backup environment variables
#[tauri::command]
pub async fn backup_envs<R: Runtime>(
	backup_name: String,
	folder: String,
	_app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	println!(
		"backup_envs: backup_name: {:?}, folder: {:?}",
		backup_name, folder
	);
	Ok(())
}

/// recover environment variables
#[tauri::command]
pub async fn recover_envs<R: Runtime>(
	file: String,
	name: String,
	_app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	println!("recover_envs: file: {:?}, name: {:?}", file, name);
	Ok(())
}

/// environment variables apply to system
#[tauri::command]
pub async fn env_apply<R: Runtime>(
	env_key: String,
	env_value: String,
	_app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	println!(
		"env_apply: env_key: {:?}, env_value: {:?}",
		env_key, env_value
	);
	Ok(())
}

/// group environment variables apply to system
#[tauri::command]
pub async fn group_env_apply<R: Runtime>(
	config_id: String,
	group_id: String,
	_app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	println!(
		"group_env_apply: config_id: {:?}, group_id: {:?}",
		config_id, group_id
	);
	Ok(())
}

/// group environment variables check from system
#[tauri::command]
pub async fn group_env_check<R: Runtime>(
	config_id: String,
	group_id: String,
	_app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	println!(
		"group_env_check: config_id: {:?}, group_id: {:?}",
		config_id, group_id
	);
	Ok(())
}

/// config check from system
#[tauri::command]
pub async fn config_check<R: Runtime>(
	config_id: String,
	_app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	println!("config_check: config_id: {:?}", config_id);
	Ok(())
}

/// config apply to system
#[tauri::command]
pub async fn config_apply<R: Runtime>(
	config_id: String,
	_app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	println!("config_apply: config_id: {:?}", config_id);
	Ok(())
}
