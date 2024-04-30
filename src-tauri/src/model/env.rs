use lombok::{Getter, Setter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getter, Setter)]
pub struct EnvInfo {
	#[serde(rename = "configId")]
	config_id: String,
	#[serde(rename = "groupId")]
	group_id: String,
	key: String,
	value: String,
	note: Option<String>,
	sort: u64,
	#[serde(rename = "isApplied")]
	is_applied: Option<bool>,
	#[serde(rename = "isSame")]
	is_same: Option<bool>,
	#[serde(rename = "currentValue")]
	current_value: Option<String>,
}

impl EnvInfo {

}
