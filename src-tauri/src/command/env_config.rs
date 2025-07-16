use super::*;
use crate::service::QueriesService;
use crate::{db::AppState, model::EnvConfig};
use tauri::State;

#[tauri::command]
pub async fn list_env_configs(state: State<'_, AppState>) -> SResult<Vec<EnvConfig>> {
	let db_conn = state.db_conn.clone();
	match QueriesService::list_env_configs(&db_conn).await {
		Ok(models) => {
			let configs: Vec<EnvConfig> = models.into_iter().map(EnvConfig::from).collect();
			Ok(Seccess::success(configs))
		}
		Err(e) => Err(Fail::fail_default()),
	}
}
