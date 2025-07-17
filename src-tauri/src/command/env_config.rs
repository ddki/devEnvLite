use super::*;
use crate::service::QueriesService;
use crate::service::MutationsService;
use crate::service::TransactionService;
use crate::model::EnvConfig;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn list_env_configs(state: State<'_, AppState>) -> SResult<Vec<EnvConfig>> {
	let db_conn = state.db_conn.clone();
	match QueriesService::list_env_configs(&db_conn).await {
		Ok(models) => {
			let configs: Vec<EnvConfig> = models.into_iter().map(EnvConfig::from).collect();
			Ok(Success::success(configs))
		}
		Err(e) => Err(Fail::fail_default()),
	}
}

#[tauri::command]
pub async fn get_env_config(
	id: String,
	state: State<'_, AppState>,
) -> SResult<EnvConfig> {
	let db_conn = state.db_conn.clone();
	match QueriesService::get_env_config(&db_conn, id).await {
		Ok(model) => {
			Ok(Success::success(EnvConfig::from(model.unwrap())))
		}
		Err(e) => Err(Fail::fail_default()),
	}
}

#[tauri::command]
pub async fn get_env_config_with_groups(
	id: String,
	state: State<'_, AppState>,
) -> SResult<EnvConfig> {
	let db_conn = state.db_conn.clone();
	match QueriesService::get_env_config_with_groups(&db_conn, id).await {
		Ok(Some(config)) => Ok(Success::success(config)),
		Ok(None) => Err(Fail::fail("404", "Config not found")),
		Err(e) => Err(Fail::fail_default()),
	}
}

#[tauri::command]
pub async fn create_env_config(
	config: EnvConfig,
	state: State<'_, AppState>,
) -> SResult<String> {
	let db_conn = state.db_conn.clone();
	match MutationsService::create_env_config(&db_conn, EnvConfig::into(config)).await {
		Ok(result) => Ok(Success::success(result)),
		Err(e) => Err(Fail::fail_default()),
	}
}

#[tauri::command]
pub async fn update_env_config(
	config: EnvConfig,
	state: State<'_, AppState>,
) -> SResult<()> {
	let db_conn = state.db_conn.clone();
	match MutationsService::update_env_config(&db_conn, EnvConfig::into(config)).await {
		Ok(_) => Ok(Success::success(())),
		Err(e) => Err(Fail::fail_default()),
	}
}

#[tauri::command]
pub async fn delete_env_config(
	id: String,
	state: State<'_, AppState>,
) -> SResult<()> {
	let db_conn = state.db_conn.clone();
	match MutationsService::delete_env_config(&db_conn, id).await {
		Ok(_) => Ok(Success::success(())),
		Err(e) => Err(Fail::fail_default()),
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
		Err(e) => Err(Fail::fail_default()),
	}
}
