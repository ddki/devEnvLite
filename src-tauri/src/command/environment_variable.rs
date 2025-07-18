use super::*;
use crate::model::EnvironmentVariable;
use crate::service::MutationsService;
use crate::service::QueriesService;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn list_environment_variables(
	group_id: String,
	state: State<'_, AppState>,
) -> SResult<Vec<EnvironmentVariable>> {
	let db_conn = state.db_conn.clone();
	match QueriesService::list_environment_variables(&db_conn, group_id).await {
		Ok(models) => {
			let variables = models.into_iter().map(EnvironmentVariable::from).collect();
			Ok(Success::success(variables))
		}
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn get_environment_variable(
	id: String,
	state: State<'_, AppState>,
) -> SResult<EnvironmentVariable> {
	let db_conn = state.db_conn.clone();
	match QueriesService::get_environment_variable(&db_conn, id).await {
		Ok(Some(model)) => Ok(Success::success(EnvironmentVariable::from(model))),
		Ok(None) => Err(Fail::fail("404", String::from("没有找到环境变量"))),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn create_environment_variable(
	variable: EnvironmentVariable,
	state: State<'_, AppState>,
) -> SResult<String> {
	let db_conn = state.db_conn.clone();
	match MutationsService::create_environment_variable(
		&db_conn,
		EnvironmentVariable::into(variable),
	)
	.await
	{
		Ok(result) => Ok(Success::success(result)),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn update_environment_variable(
	variable: EnvironmentVariable,
	state: State<'_, AppState>,
) -> SResult<()> {
	let db_conn = state.db_conn.clone();
	match MutationsService::update_environment_variable(
		&db_conn,
		EnvironmentVariable::into(variable),
	)
	.await
	{
		Ok(_) => Ok(Success::success(())),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn delete_environment_variable(id: String, state: State<'_, AppState>) -> SResult<()> {
	let db_conn = state.db_conn.clone();
	match MutationsService::delete_environment_variable(&db_conn, id).await {
		Ok(_) => Ok(Success::success(())),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}
