use std::fs::File;
use std::io::BufReader;
use std::io::Write;

use super::*;
use crate::environment_vars::get_environment_vars_manager;
use crate::environment_vars::EnvironmentVars;
use crate::environment_vars::EnvironmentVarsType;
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
pub async fn check_config_name_exists(
	config_name: String,
	exclude_config_id: Option<String>,
	state: State<'_, AppState>,
) -> SResult<bool> {
	let db_conn = state.db_conn.clone();
	let config = QueriesService::list_env_configs_by_name(&db_conn, exclude_config_id, config_name)
		.await
		.map_err(|e| {
			log::error!("检查配置名是否存在失败: {:?}", e);
			return Fail::fail_with_message(String::from("未获取到配置信息"));
		})?;
	if config.is_empty() {
		return Ok(Success::success(false));
	}
	Ok(Success::success(true))
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
					let settings = crate::model::Settings::load_from_store(&app).map_err(|e| {
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

#[tauri::command]
pub async fn import_env_config_from_file(
	file_path: String,
	config_name: String,
	state: State<'_, AppState>,
) -> SResult<()> {
	let config_file = File::open(file_path).map_err(|e| {
		log::error!("导入配置失败: 打开文件失败 {:?}", e);
		return Fail::fail_with_message(String::from("打开文件失败"));
	})?;
	let reader = BufReader::new(config_file);
	let mut config: EnvConfig = serde_json::from_reader(reader).map_err(|e| {
		log::error!("导入配置失败: 文件格式有误 {:?}", e);
		return Fail::fail_with_message(String::from("文件格式有误"));
	})?;
	config.clean_ids();
	config.name = config_name.clone();
	// 检查配置是否存在
	let db_conn = state.db_conn.clone();
	let config_exists =
		QueriesService::list_env_configs_by_name(&db_conn, None, config_name.clone())
			.await
			.map_err(|e| {
				log::error!("导入配置失败: 检查配置名是否存在失败 {:?}", e);
				return Fail::fail_with_message(String::from("检查配置名是否存在失败"));
			})?;
	if !config_exists.is_empty() {
		return Err(Fail::fail_with_message(String::from(format!(
			"配置名称 {} 已存在",
			config_name
		))));
	}
	match TransactionService::create_env_config(&db_conn, EnvConfig::into(config)).await {
		Ok(_result) => Ok(Success::success(())),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn import_env_config_from_url(
	url: String,
	config_name: String,
	state: State<'_, AppState>,
) -> SResult<()> {
	let mut config = reqwest::get(url)
		.await
		.map_err(|e| {
			log::error!("从网络导入配置失败: 获取配置失败 {:?}", e);
			Fail::fail_with_message(String::from("获取配置失败"))
		})?
		.json::<EnvConfig>()
		.await
		.map_err(|e| {
			log::error!("从网络导入配置失败: 内容格式有误 {:?}", e);
			Fail::fail_with_message(String::from("内容格式有误"))
		})?;
	config.clean_ids();
	config.name = config_name.clone();
	// 检查配置是否存在
	let db_conn = state.db_conn.clone();
	let config_exists =
		QueriesService::list_env_configs_by_name(&db_conn, None, config_name.clone())
			.await
			.map_err(|e| {
				log::error!("从网络导入配置失败: 检查配置名是否存在失败 {:?}", e);
				return Fail::fail_with_message(String::from("检查配置名是否存在失败"));
			})?;
	if !config_exists.is_empty() {
		return Err(Fail::fail_with_message(String::from(format!(
			"配置名称 {} 已存在",
			config_name
		))));
	}
	match TransactionService::create_env_config(&db_conn, EnvConfig::into(config)).await {
		Ok(_result) => Ok(Success::success(())),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn apply_env_config(id: Option<String>, state: State<'_, AppState>) -> SResult<()> {
	match id {
		Some(id) => {
			let db_conn = state.db_conn.clone();
			let config = QueriesService::get_env_config_with_groups(&db_conn, id)
				.await
				.map_err(|e| {
					log::error!("应用配置失败: 获取配置失败 {:?}", e);
					return Fail::fail_with_message(String::from("获取配置失败"));
				})?;
			match config {
				Some(config) => {
					if !config.is_active {
						return Err(Fail::fail_with_message(String::from("配置未激活")));
					}
					let variables = config.flatten_variables();
					let manager = get_environment_vars_manager(
						&EnvironmentVarsType::from_str(&config.scope)
							.map_err(|_| Fail::fail_with_message(String::from("环境变量作用域有误")))
							.unwrap(),
					);
					variables
						.into_iter()
						.map(|variable| (variable.key, variable.value))
						.for_each(|(key, value)| {
							manager.inner().set(&key, &value).unwrap();
						});
					Ok(Success::success(()))
				}
				None => Err(Fail::fail_with_message(String::from("没有找到配置")))
			}
		}
		None => {
			let db_conn = state.db_conn.clone();
			let configs = QueriesService::list_env_configs_with_group_active(&db_conn)
				.await
				.map_err(|e| {
					log::error!("应用配置失败: 获取激活配置失败 {:?}", e);
					return Fail::fail_with_message(String::from("获取激活配置失败"));
				})?;
			if configs.is_empty() {
				return Err(Fail::fail_with_message(String::from("没有激活的配置")));
			}
			configs.into_iter().for_each(|config| {
				let variables = config.flatten_variables();
				let manager = get_environment_vars_manager(
					&EnvironmentVarsType::from_str(&config.scope)
						.map_err(|_| Fail::fail_with_message(String::from("环境变量作用域有误")))
						.unwrap(),
				);
				variables
					.into_iter()
					.map(|variable| (variable.key, variable.value))
					.for_each(|(key, value)| {
						manager.inner().set(&key, &value).unwrap();
					});
			});
			Ok(Success::success(()))
		}
	}
}
