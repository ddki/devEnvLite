use lombok::{Getter, Setter};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize, Getter, Setter)]
pub struct EnvInfo {
	#[serde(rename(serialize = "configId", deserialize = "config_id"))]
	config_id: String,
	#[serde(rename(serialize = "groupId", deserialize = "group_id"))]
	group_id: String,
	key: String,
	value: String,
	note: Option<String>,
	sort: u64,
	#[serde(rename(serialize = "isApplied", deserialize = "is_applied"))]
	is_applied: bool,
	#[serde(rename(serialize = "isSame", deserialize = "is_same"))]
	is_same: bool,
	#[serde(rename(serialize = "currentValue", deserialize = "current_value"))]
	current_value: String,
}

impl EnvInfo {
	pub fn new_from_map(map: Map<String, Value>) -> Option<Self> {
		let config_id = map
			.get("configId")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_default();

		let group_id = map
			.get("groupId")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_default();

		let key = map
			.get("key")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_default();

		let value = map
			.get("value")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_default();

		let note = map
			.get("note")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string());

		let sort = map.get("sort").and_then(|v| v.as_u64()).unwrap_or_default();

		let is_applied = map
			.get("isApplied")
			.and_then(|v| v.as_bool())
			.unwrap_or_default();

		let is_same = map
			.get("isSame")
			.and_then(|v| v.as_bool())
			.unwrap_or_default();

		let current_value = map
			.get("currentValue")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_default();

		Some(EnvInfo {
			config_id,
			group_id,
			key,
			value,
			note,
			sort,
			is_applied,
			is_same,
			current_value,
		})
	}
}
