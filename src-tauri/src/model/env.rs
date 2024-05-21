use lombok::{Getter, Setter};
use serde::{Deserialize, Serialize};

use crate::environment_vars::{get_environment_vars_manager, EnvironmentVars};

use super::config::ConfigInfo;

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

	pub fn apply<R: tauri::Runtime>(&mut self, app: tauri::AppHandle<R>) -> anyhow::Result<()> {
		let mut config_info = ConfigInfo::load_from_file(&self.config_id, app.clone())?;
		let scope_enum = &config_info.get_scope_enum();
		let manager = get_environment_vars_manager(&scope_enum);

		// 应用环境变量
		match manager.inner().set(&self.get_key(), &self.get_value()) {
			Ok(_) => {
				self.set_is_applied(Some(true));
			},
			Err(_) => {
				self.set_is_applied(Some(false));
			}
		}

		// 更新配置
		if let Some(mut groups) = config_info.groups.clone() {
			let target_group = groups.iter_mut().find(|group| group.get_id().eq(self.get_group_id()));
			if let Some(env_group) = target_group {
				match env_group.update_env(self.clone()) {
					Ok(_) => {
						match config_info.update_group(env_group.clone()) {
							Ok(_) => {
								config_info.save_to_file(app.clone())?;
							},
							_ => {}
						}
					},
					_ => {}
				}
			}
		}

		Ok(())
	}
}
