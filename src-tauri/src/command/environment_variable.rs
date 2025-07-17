use super::*;
use crate::service::QueriesService;
use crate::service::MutationsService;
use crate::service::TransactionService;
use crate::model::EnvConfig;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn list_environment_variables(
    state: State<'_, AppState>,
) -> SResult<Vec<EnvironmentVariable>> {
    let db_conn = state.db_conn.clone();
    match QueriesService::list_environment_variables(&db_conn).await {
        Ok(variables) => Ok(Success::success(variables)),
        Err(e) => Err(Fail::fail_default()),
    }
}

#[tauri::command]
pub async fn get_environment_variable(
    id: String,
    state: State<'_, AppState>,
) -> SResult<EnvironmentVariable> {
    let db_conn = state.db_conn.clone();
    match QueriesService::get_environment_variable(&db_conn, id).await {
        Ok(Some(variable)) => Ok(Success::success(variable)),
        Ok(None) => Err(Fail::fail("404", "Variable not found")),
        Err(e) => Err(Fail::fail_default()),
    }
}

#[tauri::command]
pub async fn create_environment_variable(
    variable: EnvironmentVariable,
    state: State<'_, AppState>,
) -> SResult<String> {
    let db_conn = state.db_conn.clone();
    match MutationsService::create_environment_variable(&db_conn, EnvironmentVariable::into(variable)).await {
        Ok(result) => Ok(Success::success(result)),
        Err(e) => Err(Fail::fail_default()),
    }
}

#[tauri::command]
pub async fn update_environment_variable(
    variable: EnvironmentVariable,
    state: State<'_, AppState>,
) -> SResult<()> {
    let db_conn = state.db_conn.clone();
    match MutationsService::update_environment_variable(&db_conn, EnvironmentVariable::into(variable)).await {
        Ok(_) => Ok(Success::success(())),
        Err(e) => Err(Fail::fail_default()),
    }
}

#[tauri::command]
pub async fn delete_environment_variable(
    id: String,
    state: State<'_, AppState>,
) -> SResult<()> {
    let db_conn = state.db_conn.clone();
    match MutationsService::delete_environment_variable(&db_conn, id).await {
        Ok(_) => Ok(Success::success(())),
        Err(e) => Err(Fail::fail_default()),
    }
}

