use super::*;
use crate::model::VariableGroup;
use crate::service::MutationsService;
use crate::service::QueriesService;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn list_variable_groups(
	config_id: String,
	state: State<'_, AppState>,
) -> SResult<Vec<VariableGroup>> {
	let db_conn = state.db_conn.clone();
	match QueriesService::list_variable_groups(&db_conn, config_id).await {
		Ok(models) => {
			let groups = models.into_iter().map(VariableGroup::from).collect();
			Ok(Success::success(groups))
		}
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn get_variable_group(id: String, state: State<'_, AppState>) -> SResult<VariableGroup> {
	let db_conn = state.db_conn.clone();
	match QueriesService::get_variable_group(&db_conn, id).await {
		Ok(Some(group)) => Ok(Success::success(VariableGroup::from(group))),
		Ok(None) => Err(Fail::fail("404", String::from("没有找到环境变量分组"))),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn create_variable_group(
	group: VariableGroup,
	state: State<'_, AppState>,
) -> SResult<String> {
	let db_conn = state.db_conn.clone();
	match MutationsService::create_variable_group(&db_conn, VariableGroup::into(group)).await {
		Ok(result) => Ok(Success::success(result)),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn update_variable_group(
	group: VariableGroup,
	state: State<'_, AppState>,
) -> SResult<()> {
	let db_conn = state.db_conn.clone();
	match MutationsService::update_variable_group(&db_conn, VariableGroup::into(group)).await {
		Ok(_) => Ok(Success::success(())),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn delete_variable_group(id: String, state: State<'_, AppState>) -> SResult<()> {
	let db_conn = state.db_conn.clone();
	match MutationsService::delete_variable_group(&db_conn, id).await {
		Ok(_) => Ok(Success::success(())),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}
