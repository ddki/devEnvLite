use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvInfo {
	group_id: String,
	key: String,
	value: String,
	sort: u32,
	note: Option<String>,
	is_applied: bool,
}
