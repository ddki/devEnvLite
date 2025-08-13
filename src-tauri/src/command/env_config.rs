use std::io::Write;

use super::*;
use crate::model::EnvConfig;
use crate::service::MutationsService;
use crate::service::QueriesService;
use crate::service::TransactionService;
use crate::AppState;
use tauri::State;
use tauri_plugin_fs::FsExt;
use tauri_plugin_fs::OpenOptions;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub async fn list_env_configs(state: State<'_, AppState>) -> SResult<Vec<EnvConfig>> {
	let db_conn = state.db_conn.clone();
	match QueriesService::list_env_configs(&db_conn).await {
		Ok(models) => {
			let configs: Vec<EnvConfig> = models.into_iter().map(EnvConfig::from).collect();
			Ok(Success::success(configs))
		}
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn get_env_config(id: String, state: State<'_, AppState>) -> SResult<EnvConfig> {
	let db_conn = state.db_conn.clone();
	match QueriesService::get_env_config(&db_conn, id).await {
		Ok(model) => Ok(Success::success(EnvConfig::from(model.unwrap()))),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

// 尚未使用
#[tauri::command]
pub async fn get_env_config_with_groups(
	id: String,
	state: State<'_, AppState>,
) -> SResult<EnvConfig> {
	let db_conn = state.db_conn.clone();
	match QueriesService::get_env_config_with_groups(&db_conn, id).await {
		Ok(Some(config)) => Ok(Success::success(config)),
		Ok(None) => Err(Fail::fail("404", String::from("没有找到配置"))),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn create_env_config(config: EnvConfig, state: State<'_, AppState>) -> SResult<String> {
	let mut config = config;
	config.id = String::default();
	let db_conn = state.db_conn.clone();
	match MutationsService::create_env_config(&db_conn, EnvConfig::into(config)).await {
		Ok(result) => Ok(Success::success(result)),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn update_env_config(config: EnvConfig, state: State<'_, AppState>) -> SResult<()> {
	let db_conn = state.db_conn.clone();
	match MutationsService::update_env_config(&db_conn, EnvConfig::into(config)).await {
		Ok(_) => Ok(Success::success(())),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn delete_env_config(id: String, state: State<'_, AppState>) -> SResult<()> {
	let db_conn = state.db_conn.clone();
	match MutationsService::delete_env_config(&db_conn, id).await {
		Ok(_) => Ok(Success::success(())),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn create_env_config_transaction(
	config: EnvConfig,
	state: State<'_, AppState>,
) -> SResult<String> {
	let db_conn = state.db_conn.clone();
	match TransactionService::create_env_config(&db_conn, EnvConfig::into(config)).await {
		Ok(result) => Ok(Success::success(result)),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn delete_env_config_transaction(id: String, state: State<'_, AppState>) -> SResult<()> {
	let db_conn = state.db_conn.clone();
	match TransactionService::delete_env_config(&db_conn, id).await {
		Ok(_) => Ok(Success::success(())),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn check_variable_key_exists_in_config(
	group_id: String,
	exclude_variable_id: Option<String>,
	key: String,
	state: State<'_, AppState>,
) -> SResult<bool> {
	let db_conn = state.db_conn.clone();
	let group = QueriesService::get_variable_group(&db_conn, group_id)
		.await
		.map_err(|e| {
			log::error!("检查变量名是否存在在配置中失败: {:?}", e);
			return Fail::fail_with_message(String::from("未获取到分组信息"));
		})?;
	if group.is_none() {
		return Err(Fail::fail("404", String::from("没有找到分组")));
	}
	let group = group.unwrap();
	match QueriesService::get_env_config_with_groups(&db_conn, group.config_id).await {
		Ok(Some(config)) => {
			let key_exist = config
				.groups
				.iter()
				.flat_map(|groups| groups.iter())
				.flat_map(|group| group.variables.iter())
				.flat_map(|vars| vars.iter())
				.filter(|var| match &exclude_variable_id {
					Some(id) => var.id != *id,
					None => true,
				})
				.map(|var| var.key.clone())
				.collect::<Vec<String>>()
				.iter()
				.any(|db_key| key.contains(db_key));

			Ok(Success::success(key_exist))
		}
		Ok(None) => Err(Fail::fail("404", String::from("没有找到配置"))),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn export_env_config<R: tauri::Runtime>(
	id: String,
	state: State<'_, AppState>,
	app: tauri::AppHandle<R>,
) -> SResult<()> {
	match self::get_env_config_with_groups(id, state).await {
		Ok(config) => match config.code {
			Some(code) => {
				if code == "200" {
					// 转换为json字符串
					let config_data = config.data.unwrap();
					let config_json_str = serde_json::to_string(&config_data).unwrap();
					// 存储为文件
					let settings =
						crate::model::Settings::load_from_store(app.clone()).map_err(|e| {
							log::error!("导出配置失败: 获取应用配置失败 {:?}", e);
							return Fail::fail_with_message(String::from("获取应用配置失败"));
						})?;
					let date = chrono::Local::now().format("%Y%m%d%H%M%S%f");
					let config_file_name =
						format!("导出-{}-{}-{}.json", config_data.name, config_data.id, date);
					let config_file_path =
						std::path::Path::new(&settings.get_data_dir()).join(config_file_name);

					let opts = OpenOptions::new()
						.write(true)
						.create(true)
						.truncate(true)
						.to_owned();
					app.clone()
						.fs()
						.open(config_file_path.clone(), opts)
						.map_err(|e| {
							log::error!("导出配置失败: 打开文件失败 {:?}", e);
							return Fail::fail_with_message(String::from("打开文件失败"));
						})
						.and_then(|mut file| {
							file.write_all(config_json_str.as_bytes()).map_err(|e| {
								log::error!("导出配置失败: 写入文件失败 {:?}", e);
								return Fail::fail_with_message(String::from("写入文件失败"));
							})
						})
						.map_err(|e| {
							log::error!("导出配置失败: 写入文件失败 {:?}", e);
							return Fail::fail_with_message(String::from("写入文件失败"));
						})?;
					// 打开文件
					let opener = app.opener();
					opener
						.reveal_item_in_dir(&config_file_path.clone().to_string_lossy().to_string())
						.inspect_err(|e| {
							log::error!("导出配置失败: {}", e);
						})
						.map_err(|e| Fail::fail_with_message(e.to_string()))?;
					return Ok(Success::success(()));
				}
			}
			None => {
				return Err(Fail::fail("404", String::from("没有找到配置")));
			}
		},
		Err(e) => {
			return Err(Fail::fail_with_message(e.message.unwrap_or_default()));
		}
	}
	Ok(Success::success(()))
}
