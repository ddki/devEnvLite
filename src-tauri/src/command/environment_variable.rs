use super::*;
use crate::model::EnvironmentVariable;
use crate::service::QueriesService;
use crate::service::TransactionService;
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
	group_id: String,
	variable: EnvironmentVariable,
	state: State<'_, AppState>,
) -> SResult<String> {
	let db_conn = state.db_conn.clone();
	match TransactionService::create_environment_variable(
		&db_conn,
		group_id,
		variable,
	)
	.await
	{
		Ok(variable_id) => Ok(Success::success(variable_id)),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn update_environment_variable(
	group_id: String,
	variable: EnvironmentVariable,
	state: State<'_, AppState>,
) -> SResult<()> {
	let db_conn = state.db_conn.clone();
	match TransactionService::update_environment_variable(
		&db_conn,
		group_id,
		EnvironmentVariable::into(variable),
	)
	.await
	{
		Ok(_) => Ok(Success::success(())),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn delete_environment_variable(
	group_id: String,
	id: String,
	state: State<'_, AppState>,
) -> SResult<()> {
	let db_conn = state.db_conn.clone();
	match TransactionService::delete_environment_variable(&db_conn, group_id, id).await {
		Ok(_) => Ok(Success::success(())),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}
