use lombok::{Getter, Setter};
use serde::{Deserialize, Serialize};

use crate::environment_vars::{get_environment_vars_manager, EnvironmentVars, EnvironmentVarsType};

use super::env::EnvInfo;

#[derive(Debug, Clone, Serialize, Deserialize, Getter, Setter)]
pub struct GroupInfo {
	#[serde(rename = "configId")]
	config_id: String,
	id: String,
	name: String,
	note: Option<String>,
	sort: u64,
	pub envs: Option<Vec<EnvInfo>>,
	#[serde(rename = "envAppliedCount")]
	env_applied_count: u64,
	#[serde(rename = "envNotAppliedCount")]
	env_not_applied_count: u64,
}

impl GroupInfo {

	fn apply(&self, env_type: &EnvironmentVarsType) -> anyhow::Result<()> {
		let manager = get_environment_vars_manager(env_type);
		if let Some(envs) = &self.envs {
			envs.iter().for_each(|env| {
				let _ = manager.inner().set(&env.get_key(), &env.get_value());
			})
		}
		Ok(())
	}
}
