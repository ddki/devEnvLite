use std::{collections::{HashMap, HashSet}, path::PathBuf};

use anyhow::Error;
use log::{debug, info};
use tauri::{AppHandle, Manager, Runtime};

use crate::{
	environment_vars::{get_environment_vars_manager, EnvironmentVars, EnvironmentVarsType},
	error::AppError,
	model::{self, config::ConfigInfo},
};

#[tauri::command]
pub fn close_splashscreen(app: AppHandle) {
	debug!("close_splashscreen");
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
	info!("app_config_dir = {:?}", app_config_dir);
	info!("app_cache_dir = {:?}", app_cache_dir);
	info!("app_data_dir = {:?}", app_data_dir);
	info!("app_log_dir = {:?}", app_log_dir);
	info!("app_local_data_dir = {:?}", app_local_data_dir);
	// info!("font_dir = {:?}", font_dir);
	info!("home_dir = {:?}", home_dir);
	info!("cache_dir = {:?}", cache_dir);
	info!("data_dir = {:?}", data_dir);
	info!("config_dir = {:?}", config_dir);
	info!("local_data_dir = {:?}", local_data_dir);
	let mut config_path = app_data_dir.clone();
	config_path.push("config");

	let mut config_ids: Vec<String> = vec![];
	if let Ok(entries) = std::fs::read_dir(config_path) {
		for entry in entries.flatten() {
			let file_path = entry.path();
			if let Some(extension) = file_path.extension() {
				info!("extension: {:?}", extension);
				info!("file_name: {:?}", file_path.file_name());
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

/// get environment variables
#[tauri::command]
pub fn get_envs<R: Runtime>(
	scope: String,
	_app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<HashMap<String, String>, AppError> {
	debug!("get_envs scope: {:?}", scope);
	if EnvironmentVarsType::SYSTEM.to_string().eq(&scope) {
		let manager = get_environment_vars_manager(&EnvironmentVarsType::SYSTEM);
		let system_envs = manager.inner().read_envs()?;
		return Ok(system_envs);
	}
	if EnvironmentVarsType::USER.to_string().eq(&scope) {
		let manager = get_environment_vars_manager(&EnvironmentVarsType::USER);
		let user_envs = manager.inner().read_envs()?;
		return Ok(user_envs);
	}
	Ok(HashMap::new())
}

#[tauri::command]
pub async fn get_keys<R: Runtime>(
	scopes: Vec<String>,
	_app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<Vec<String>, AppError> {
	debug!("get_keys: scopes: {:?}", scopes);
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
	debug!("get_keys: keys: {:?}", sort_keys);
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
	debug!("collate_envs: keys: {:?}, scopes: {:?}", keys, scopes);
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
	debug!(
		"backup_envs: backup_name: {:?}, folder: {:?}",
		backup_name, folder
	);
	// todo
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
	debug!("recover_envs: file: {:?}, name: {:?}", file, name);
	// todo
	Ok(())
}

/// environment variables apply to system
#[tauri::command]
pub async fn env_apply<R: Runtime>(
	config_id: String,
	group_id: String,
	env_key: String,
	env_value: String,
	app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	debug!(
		"env_apply: config_id: {:?}, group_id: {:?}, env_key: {:?}, env_value: {:?}",
		config_id, group_id, env_key, env_value
	);
	let mut env =
		model::config::ConfigInfo::get_group_env(&config_id, &group_id, &env_key, app.clone())?;
	env.apply(app.clone())?;
	Ok(())
}

/// group environment variables apply to system
#[tauri::command]
pub async fn group_env_apply<R: Runtime>(
	config_id: String,
	group_id: String,
	app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	debug!(
		"group_env_apply: config_id: {:?}, group_id: {:?}",
		config_id, group_id
	);

	let mut group = model::config::ConfigInfo::get_group(&config_id, &group_id, app.clone())?;
	group.apply(app.clone())?;
	Ok(())
}

/// config check from system
#[tauri::command]
pub async fn config_check<R: Runtime>(
	config_id: String,
	app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	debug!("config_check: config_id: {:?}", config_id);
	let mut config_info = model::config::ConfigInfo::load_from_file(&config_id, app.clone())
		.expect("load config failed!");
	debug!("config_check: config_info: {:?}", config_info);
	Ok(config_info.check(app.clone())?)
}

/// config apply to system
#[tauri::command]
pub async fn config_apply<R: Runtime>(
	config_id: String,
	app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	debug!("config_apply: config_id: {:?}", config_id);
	let mut store_config = model::config::ConfigInfo::load_from_file(&config_id, app.clone())?;
	Ok(store_config.apply(app.clone())?)
}

/// get config by id
#[tauri::command]
pub async fn get_config<R: Runtime>(
	config_id: String,
	app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<ConfigInfo, AppError> {
	debug!("get config id: {:?}", config_id);
	let config_info = model::config::ConfigInfo::load_from_file(&config_id, app)?;
	Ok(config_info)
}

/// save config
#[tauri::command]
pub async fn save_config<R: Runtime>(
	config_info: ConfigInfo,
	app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	debug!("save config: config_info: {:?}", config_info);
	let mut config = config_info.clone();
	config.save_to_file(app.clone())?;
	Ok(())
}

#[tauri::command]
pub async fn remove_config<R: Runtime>(
	config_id: String,
	app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	debug!("remove config: id: {:?}", config_id);
	model::config::ConfigInfo::remove_file(&config_id, app.clone())?;
	Ok(())
}

#[tauri::command]
pub async fn config_export<R: Runtime>(
	config_id: String,
	app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	let config_path = model::config::ConfigInfo::get_path(&config_id, app.clone())?;
	let export_dir = app
		.path()
		.download_dir()
		.expect("system download dir not found!");
	std::fs::copy(config_path, export_dir.join(config_id + ".json"))
		.expect_err("export config failed!");
	Ok(())
}

#[tauri::command]
pub async fn import_config_form_file<R: Runtime>(
	name: String,
	path: String,
	app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	let file_path = PathBuf::from(path);
	if !file_path.exists() {
		return Err(Error::msg("file not found!"));
	}
	Ok(())
}

#[tauri::command]
pub async fn import_config_form_url<R: Runtime>(
	name: String,
	url: String,
	app: tauri::AppHandle<R>,
	_window: tauri::Window<R>,
) -> Result<(), AppError> {
	Ok(())
}
