use anyhow::{Error, Result};
use log::info;
use lombok::{Getter, Setter};
use serde::{Deserialize, Serialize};
use tauri::Manager;

use crate::environment_vars::{get_environment_vars_manager, EnvironmentVars, EnvironmentVarsType};

use super::{env::EnvInfo, group::GroupInfo};

#[derive(Debug, Clone, Serialize, Deserialize, Getter, Setter)]
pub struct ConfigInfo {
	id: String,
	scope: String,
	name: String,
	note: Option<String>,
	sort: u64,
	#[serde(rename = "groupEnvs")]
	pub groups: Option<Vec<GroupInfo>>,
}

impl ConfigInfo {
	pub fn load_from_file<R: tauri::Runtime>(id: &str, app: tauri::AppHandle<R>) -> Result<Self> {
		let path = app
			.path()
			.app_data_dir()
			.unwrap()
			.join("config")
			.join(format!("{}.json", id));
		info!("load config from file path: {:#?}", path);
		if !path.exists() {
			return Err(Error::msg("not found config"));
		}
		let content = std::fs::read_to_string(path)?;
		let config = serde_json::from_str(&content).expect("config file format error");
		info!("load config from file config: {:#?}", config);
		Ok(config)
	}

	pub fn save_to_file<R: tauri::Runtime>(&self, app: tauri::AppHandle<R>) -> Result<()> {
		let path = app
			.path()
			.app_data_dir()
			.unwrap()
			.join("config")
			.join(format!("{}.json", self.get_id()));
		info!("save config from file path: {:#?}", path);
		if !path.exists() {
			std::fs::create_dir_all(path.parent().unwrap())?;
		}
		let file = std::fs::File::create(path)?;
		serde_json::to_writer(file, self).expect("save config failed");

		Ok(())
	}

	pub fn remove_file<R: tauri::Runtime>(id: &str, app: tauri::AppHandle<R>) -> Result<()> {
		let path = app
			.path()
			.app_data_dir()
			.unwrap()
			.join("config")
			.join(format!("{}.json", id));
		info!("load config from file path: {:#?}", path);
		if !path.exists() {
			return Err(Error::msg("not found config"));
		}
		std::fs::remove_file(path)?;
		Ok(())
	}

	fn get_scope_enum(&self) -> EnvironmentVarsType {
		EnvironmentVarsType::from_str(self.scope.as_str()).expect("not found scope")
	}

	pub fn apply(&self) -> anyhow::Result<()> {
		let type_enum = self.get_scope_enum();
		let manager = get_environment_vars_manager(&type_enum);

		match self.groups.clone() {
			Some(groups) => {
				let envs = groups
					.iter()
					.map(|group| group.get_envs().as_ref().unwrap().clone())
					.flatten()
					.collect::<Vec<EnvInfo>>();
				envs.iter().for_each(|env| {
					let _ = manager.inner().set(env.get_key(), env.get_value());
				})
			}
			None => {}
		}
		Ok(())
	}

	pub fn check<R: tauri::Runtime>(&mut self, app: tauri::AppHandle<R>) -> anyhow::Result<()> {
		let type_enum = self.get_scope_enum();
		let manager = get_environment_vars_manager(&type_enum);
		let system_envs = manager
			.inner()
			.read_envs()
			.expect("get system environment variables failed!");

		if let Some(groups) = &mut self.groups {
			for group in groups.iter_mut() {
				if let Some(envs) = group.envs.as_mut() {
					for env in envs.iter_mut() {
						let key = env.get_key();
						let value = env.get_value();
						let system_env_value = system_envs.get(key).unwrap();
						if system_env_value.eq(value) {
							env.set_is_same(Some(true));
							env.set_current_value(Some(system_env_value.to_string()));
						} else {
							match manager.inner().set(key, value) {
								Ok(_) => {
									env.set_is_applied(true);
								}
								Err(_) => {}
							}
						}
					}
				}
			}
		}
		self.save_to_file(app)
	}
}
