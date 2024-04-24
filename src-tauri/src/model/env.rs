use lombok::Getter;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize, Getter)]
pub struct EnvInfo {
	#[serde(rename(serialize = "groupId", deserialize = "group_id"))]
	group_id: String,
	key: String,
	value: String,
	note: Option<String>,
	sort: u64,
	#[serde(rename(serialize = "isApplied", deserialize = "is_applied"))]
	is_applied: bool,
}

impl EnvInfo {
	pub fn new_from_map(map: Map<String, Value>) -> Option<Self> {
		let group_id = map
			.get("groupId")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_else(|| "".to_string());
		let key = map
			.get("key")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_else(|| "".to_string());
		let value = map
			.get("value")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_else(|| "".to_string());
		let note = map
			.get("note")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string());
		let sort = map.get("sort").and_then(|v| v.as_u64()).unwrap_or(0);
		let is_applied = map
			.get("isApplied")
			.and_then(|v| v.as_bool())
			.unwrap_or(false);

		Some(EnvInfo {
			group_id,
			key,
			value,
			note,
			sort,
			is_applied,
		})
	}
}
