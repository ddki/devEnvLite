use log::info;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_plugin_store::StoreBuilder;

use super::group::GroupInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigInfo {
	id: String,
	name: String,
	note: Option<String>,
	sort: u64,
	groups: Option<Vec<GroupInfo>>,
}

impl ConfigInfo {
	pub fn load_from_store<R: tauri::Runtime>(id: &str, app: tauri::AppHandle<R>) -> Option<Self> {
		let path = app
			.path()
			.app_data_dir()
			.unwrap()
			.join("config")
			.join(format!("{}.json", id));
		if !path.exists() {
			return None;
		}
		info!("load config from store path: {:#?}", path);
		let mut store = StoreBuilder::new(path).build(app);
		store.load().expect("load config from store failed");
		info!("load config from store store: {:#?}", store);
		let id = store
			.get("id")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_else(|| "".to_string());
		let name = store
			.get("name")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_else(|| "".to_string());
		let note = store
			.get("note")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string());
		let sort = store.get("sort").and_then(|v| v.as_u64()).unwrap_or(0);

		info!(
			"load config from store id: {:#?}, name: {:#?}, note: {:#?}, sort: {:#?}",
			id, name, note, sort
		);
		store.get("groupEnvs").and_then(|v| v.as_array()).map(|arr| {
			let groups = arr
				.iter()
				.map(|v| GroupInfo::new_from_map(v.as_object().unwrap().clone()))
				.collect::<Option<Vec<GroupInfo>>>();
			info!("load config from store groups: {:#?}", groups);
			ConfigInfo {
				id,
				name,
				note,
				sort,
				groups,
			}
		})
	}
}
