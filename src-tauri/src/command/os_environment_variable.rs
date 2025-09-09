use std::{
	collections::{HashMap, HashSet}, fs::File, io::{BufReader, Write}
};

use log::info;
use tauri_plugin_fs::{FsExt as _, OpenOptions};
use tauri_plugin_opener::OpenerExt;

use crate::environment_vars::{get_environment_vars_manager, EnvironmentVars, EnvironmentVarsType};

use super::*;

/// ## 获取操作系统的环境变量
/// scope: 环境变量类型，USER 或 SYSTEM
#[tauri::command]
pub async fn get_os_environment_variables(scope: String) -> SResult<HashMap<String, String>> {
	if EnvironmentVarsType::SYSTEM.to_string().eq(&scope) {
		let manager = get_environment_vars_manager(&EnvironmentVarsType::SYSTEM);
		match manager.inner().read_envs() {
			Ok(system_envs) => return Ok(Success::success(system_envs)),
			Err(e) => {
				return Err(Fail::fail_with_message(format!(
					"获取系统环境变量失败: {}",
					e
				)))
			}
		}
	}
	if EnvironmentVarsType::USER.to_string().eq(&scope) {
		let manager = get_environment_vars_manager(&EnvironmentVarsType::USER);
		match manager.inner().read_envs() {
			Ok(user_envs) => return Ok(Success::success(user_envs)),
			Err(e) => {
				return Err(Fail::fail_with_message(format!(
					"获取用户环境变量失败: {}",
					e
				)))
			}
		}
	}
	Ok(Success::success(HashMap::new()))
}

/// ## 获取操作系统的环境变量键
/// scopes: 环境变量类型列表，包含 USER 和 SYSTEM
#[tauri::command]
pub async fn get_os_environment_variable_keys(scopes: Vec<String>) -> SResult<Vec<String>> {
	let mut keys = HashSet::new();
	if scopes.contains(&EnvironmentVarsType::SYSTEM.to_string()) {
		let manager = get_environment_vars_manager(&EnvironmentVarsType::SYSTEM);
		match manager.inner().get_keys() {
			Ok(system_keys) => keys.extend(system_keys),
			Err(e) => {
				return Err(Fail::fail_with_message(format!(
					"获取系统环境变量键失败: {}",
					e
				)))
			}
		}
	}
	if scopes.contains(&EnvironmentVarsType::USER.to_string()) {
		let manager = get_environment_vars_manager(&EnvironmentVarsType::USER);
		match manager.inner().get_keys() {
			Ok(user_keys) => keys.extend(user_keys),
			Err(e) => {
				return Err(Fail::fail_with_message(format!(
					"获取用户环境变量键失败: {}",
					e
				)))
			}
		}
	}
	let mut sorted_keys: Vec<String> = keys.into_iter().collect();
	sorted_keys.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
	Ok(Success::success(sorted_keys))
}

/// ## 整理环境变量
/// keys: 要整理的环境变量键列表
/// scopes: 环境变量类型列表，包含 USER 和 SYSTEM
#[tauri::command]
pub async fn collate_os_environment_variables(
	keys: Vec<String>,
	scopes: Vec<String>,
) -> SResult<()> {
	if scopes.contains(&EnvironmentVarsType::SYSTEM.to_string()) {
		let manager = get_environment_vars_manager(&EnvironmentVarsType::SYSTEM);
		manager
			.inner()
			.collate(keys.clone())
			.map_err(|e| Fail::fail_with_message(format!("整理系统环境变量失败: {}", e)))?;
	}
	if scopes.contains(&EnvironmentVarsType::USER.to_string()) {
		let manager = get_environment_vars_manager(&EnvironmentVarsType::USER);
		manager
			.inner()
			.collate(keys)
			.map_err(|e| Fail::fail_with_message(format!("整理用户环境变量失败: {}", e)))?;
	}
	Ok(Success::success(()))
}

/// ## 备份操作系统的环境变量
/// backup_name: 备份名称
#[tauri::command]
pub async fn backup_os_environment_variables<R: tauri::Runtime>(
	backup_name: String,
	app: tauri::AppHandle<R>,
) -> SResult<()> {
	info!("备份操作系统环境变量: {}", backup_name);
	let settings = crate::model::Settings::load_from_store(&app).map_err(|e| {
		log::error!("备份操作系统环境变量失败: 获取应用配置失败 {:?}", e);
		return Fail::fail_with_message(String::from("获取应用配置失败"));
	})?;
	let setting_backup_dir = std::path::Path::new(settings.get_env_backup_dir());

	let now = chrono::Local::now();
	let backup_time = now.format("%Y%m%d%H%M%S").to_string();
	for var_type in [EnvironmentVarsType::SYSTEM, EnvironmentVarsType::USER] {
		let manager = get_environment_vars_manager(&var_type);
		let envs = manager.inner().read_envs().map_err(|e| {
			Fail::fail_with_message(format!("读取 {:?} 环境变量失败: {}", var_type, e))
		})?;
		let envs_json_str = serde_json::to_string(&envs).map_err(|e| {
			Fail::fail_with_message(format!("序列化 {:?} 环境变量失败: {}", var_type, e))
		})?;
		let backup_name = format!("{}_{}_{}.json", backup_name, var_type.as_str(), backup_time);

		let backup_file_path = setting_backup_dir.join(backup_name);
		let opts = OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.to_owned();
		app.clone()
			.fs()
			.open(backup_file_path.clone(), opts)
			.map_err(|e| {
				log::error!("备份操作系统环境变量失败: 打开文件失败 {:?}", e);
				return Fail::fail_with_message(String::from("打开文件失败"));
			})
			.and_then(|mut file| {
				file.write_all(envs_json_str.as_bytes()).map_err(|e| {
					log::error!("备份操作系统环境变量失败: 写入文件失败 {:?}", e);
					return Fail::fail_with_message(String::from("写入文件失败"));
				})
			})
			.map_err(|e| {
				log::error!("备份操作系统环境变量失败: 写入文件失败 {:?}", e);
				return Fail::fail_with_message(String::from("写入文件失败"));
			})?;
	}
	let opener = app.opener();
	opener
		.reveal_item_in_dir(setting_backup_dir.to_string_lossy().to_string())
		.inspect_err(|e| {
			log::error!("备份操作系统环境变量失败: {}", e);
		})
		.map_err(|e| Fail::fail_with_message(e.to_string()))?;
	Ok(Success::success(()))
}

/// ## 恢复操作系统的环境变量
/// backup_file: 备份文件路径
/// name: 新配置名称
#[tauri::command]
pub async fn recover_os_environment_variables(
	backup_file: String,
	scope: String,
) -> SResult<()> {
	info!(
		"恢复操作系统环境变量: scope: {}, backup_file: {}",
		scope, backup_file
	);
	let file = File::open(backup_file).map_err(|e| {
		log::error!("恢复操作系统环境变量失败: 打开文件失败 {:?}", e);
		return Fail::fail_with_message(String::from("打开文件失败"));
	})?;
	let reader = BufReader::new(file);
	let env_map: HashMap<String, String> = serde_json::from_reader(reader).map_err(|e| {
		log::error!("恢复操作系统环境变量失败: 文件格式有误 {:?}", e);
		return Fail::fail_with_message(String::from("文件格式有误"));
	})?;
	let var_type = EnvironmentVarsType::from_str(&scope).map_err(|e| {
		Fail::fail_with_message(format!(
			"恢复操作系统环境变量失败: 无效的环境变量类型 {}",
			e
		))
	})?;
	let manager = get_environment_vars_manager(&var_type);
	for (key, value) in env_map.iter() {
		manager
			.inner()
			.set(key, value)
			.map_err(|e| Fail::fail_with_message(format!("整理系统环境变量失败: {}", e)))?;
	}
	Ok(Success::success(()))
}
