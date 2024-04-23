use serde::{Deserialize, Serialize};
use tauri_plugin_store::Store;

use super::group::GroupInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigInfo {
	id: String,
	name: String,
	note: Option<String>,
	sort: u64,
	groups: Option<Vec<GroupInfo>>,
}

// impl ConfigInfo {
// 	fn load_from_store<R: tauri::Runtime>(
// 		store: &Store<R>,
// 	) -> Result<Self, Box<dyn std::error::Error>> {
// 		let id = store.get("id").and_then(|v| v.as_str()).unwrap_or("");
// 		let name = store.get("name").and_then(|v| v.as_str()).unwrap_or("");
// 		let note = store.get("note").and_then(|v| v.as_str());
// 		let sort = store.get("sort").and_then(|v| v.as_u64()).unwrap_or(0);
// 		let groups = store.get("groups").and_then(|v| v.as_array()).unwrap();
// 		let objs = groups.iter().map(|item| item.as_object()).collect();
// 	}
// }
