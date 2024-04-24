use lombok::Getter;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::env::EnvInfo;

#[derive(Debug, Clone, Serialize, Deserialize, Getter)]
pub struct GroupInfo {
	#[serde(rename(serialize = "configId", deserialize = "config_id"))]
	config_id: String,
	id: String,
	name: String,
	note: Option<String>,
	sort: u64,
	envs: Option<Vec<EnvInfo>>,
	#[serde(rename(serialize = "envAppliedCount", deserialize = "env_applied_count"))]
	env_applied_count: u64,
	#[serde(rename(serialize = "envNotAppliedCount", deserialize = "env_not_applied_count"))]
	env_not_applied_count: u64,
}

impl GroupInfo {
	pub fn new_from_map(map: Map<String, Value>) -> Option<Self> {
		let config_id = map
			.get("configId")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_else(|| "".to_string());
		let id = map
			.get("id")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_else(|| "".to_string());
		let name = map
			.get("name")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_else(|| "".to_string());
		let note = map
			.get("note")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string());
		let sort = map.get("sort").and_then(|v| v.as_u64()).unwrap_or(0);

		map.get("envs").and_then(|v| v.as_array()).map(|arr| {
			let envs = arr
				.iter()
				.map(|v| EnvInfo::new_from_map(v.as_object().unwrap().clone()))
				.collect::<Option<Vec<EnvInfo>>>();
			let env_applied_count = envs.clone()
				.and_then(|v| Some(v.iter().filter(|e| *e.get_is_applied()).count() as u64))
				.unwrap_or(0);
			let env_not_applied_count = envs.clone()
				.and_then(|v| Some(v.len() as u64 - env_applied_count))
				.unwrap_or(0);

			GroupInfo {
				config_id,
				id,
				name,
				note,
				sort,
				envs,
				env_applied_count,
				env_not_applied_count,
			}
		})
	}
}
