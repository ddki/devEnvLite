use std::collections::{HashMap, HashSet};

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
pub async fn get_os_environment_variable_keys(
    scopes: Vec<String>,
) -> SResult<Vec<String>> {
    let mut keys = HashSet::new();
    if scopes.contains(&EnvironmentVarsType::SYSTEM.to_string()) {
        let manager = get_environment_vars_manager(&EnvironmentVarsType::SYSTEM);
        match manager.inner().get_keys() {
            Ok(system_keys) => keys.extend(system_keys),
            Err(e) => return Err(Fail::fail_with_message(format!("获取系统环境变量键失败: {}", e))),
        }
    }
    if scopes.contains(&EnvironmentVarsType::USER.to_string()) {
        let manager = get_environment_vars_manager(&EnvironmentVarsType::USER);
        match manager.inner().get_keys() {
            Ok(user_keys) => keys.extend(user_keys),
            Err(e) => return Err(Fail::fail_with_message(format!("获取用户环境变量键失败: {}", e))),
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
		manager.inner().collate(keys.clone()).map_err(|e| {
			Fail::fail_with_message(format!("整理系统环境变量失败: {}", e))
		})?;
	}
	if scopes.contains(&EnvironmentVarsType::USER.to_string()) {
		let manager = get_environment_vars_manager(&EnvironmentVarsType::USER);
		manager.inner().collate(keys).map_err(|e| {
			Fail::fail_with_message(format!("整理用户环境变量失败: {}", e))
		})?;
	}
	Ok(Success::success(()))
}

/// ## 备份操作系统的环境变量
/// backup_name: 备份名称
#[tauri::command]
pub async fn backup_os_environment_variables(backup_name: String) -> SResult<()> {
	// todo
	Ok(Success::success(()))
}

/// ## 恢复操作系统的环境变量
/// backup_file: 备份文件路径
/// name: 新配置名称
#[tauri::command]
pub async fn recover_os_environment_variables(
	backup_file: String,
	name: String,
) -> SResult<()> {
	// todo
	Ok(Success::success(()))
}
