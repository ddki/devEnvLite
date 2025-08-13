use tauri::Runtime;

use super::*;
use crate::model::Settings;

#[tauri::command]
pub async fn get_settings<R: Runtime>(app: tauri::AppHandle<R>) -> SResult<Settings> {
	match Settings::load_from_store(&app) {
		Ok(settings) => Ok(Success::success(settings)),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}

#[tauri::command]
pub async fn save_settings<R: Runtime>(
	app: tauri::AppHandle<R>,
	settings: Settings,
) -> SResult<()> {
	match settings.save_to_store(&app) {
		Ok(()) => Ok(Success::success(())),
		Err(e) => Err(Fail::fail_with_message(e.to_string())),
	}
}
