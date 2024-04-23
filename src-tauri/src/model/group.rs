use serde::{Deserialize, Serialize};

use super::env::EnvInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInfo {
	config_id: String,
	id: String,
	name: String,
	note: Option<String>,
	sort: u32,
	envs: Option<Vec<EnvInfo>>,
	env_applied_count: u32,
	env_not_applied_count: u32,
}
