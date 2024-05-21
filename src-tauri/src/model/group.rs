use lombok::{Getter, Setter};
use serde::{Deserialize, Serialize};

use crate::environment_vars::{get_environment_vars_manager, EnvironmentVars};

use super::{config::ConfigInfo, env::EnvInfo};

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

	pub fn update_env(&mut self, new_env: EnvInfo) -> anyhow::Result<bool> {
		if let Some(envs) = self.envs.as_mut() {
			for e in envs.iter_mut() {
				if e.get_key() == new_env.get_key() {
					*e = new_env;
					return Ok(true);
				}
			}
		} else {
			self.envs = Some(vec![new_env]);
			return Ok(true);
		}
		Ok(false)
	}

	pub fn apply<R: tauri::Runtime>(&mut self, app: tauri::AppHandle<R>) -> anyhow::Result<()> {
		let mut config_info = ConfigInfo::load_from_file(&self.config_id, app.clone())?;
		let scope_enum = config_info.get_scope_enum();
		let manager = get_environment_vars_manager(&scope_enum);

		// 应用分组环境变量到系统中
		if let Some(envs) = self.envs.as_mut() {
			for env in envs.iter_mut() {
				match manager.inner().set(&env.get_key(), &env.get_value()) {
					Ok(_) => {
						env.set_is_applied(Some(true));
					}
					Err(_e) => {
						env.set_is_applied(Some(false));
					}
				}
			}
		}

		// 更新配置信息
		match config_info.update_group(self.clone()) {
			Ok(true) => {
				config_info.save_to_file(app.clone())?;
			},
			_ => {}
		}
		Ok(())
	}
}
