use anyhow::{Error, Result};
use log::{info, log};
use lombok::{Getter, Setter};
use serde::{Deserialize, Serialize};
use tauri::Manager;

use crate::environment_vars::{get_environment_vars_manager, EnvironmentVars, EnvironmentVarsType};

use super::
	group::GroupInfo
;

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

	pub fn save_to_file<R: tauri::Runtime>(&mut self, app: tauri::AppHandle<R>) -> Result<()> {
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
		if let Some(groups) = &mut self.groups {
			for group in groups.iter_mut() {
				if let Some(envs) = group.get_envs().clone() {
					let applied_count = envs.iter().filter(|x| {
						match x.get_is_applied() {
							Some(is_applied) => *is_applied,
							None => false,
						}
					}).count() as u64;
					group.set_env_applied_count(applied_count);
					let not_applied_count = (envs.len() as u64) - applied_count;
					group.set_env_not_applied_count(not_applied_count);
				}
			}
		}

		info!("save config from file config: {:#?}", self);
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

	pub fn get_scope_enum(&self) -> EnvironmentVarsType {
		EnvironmentVarsType::from_str(self.scope.as_str()).expect("not found scope")
	}

	pub fn apply<R: tauri::Runtime>(&mut self, app: tauri::AppHandle<R>) -> anyhow::Result<()> {
		let type_enum = self.get_scope_enum();
		let manager = get_environment_vars_manager(&type_enum);

		if let Some(groups) = &mut self.groups {
			for group in groups.iter_mut() {
				if let Some(envs) = group.envs.as_mut() {
					for env in envs.iter_mut() {
						match manager.inner().set(env.get_key(), env.get_value()) {
							Ok(_) => {
								env.set_is_applied(Some(true));
							}
							Err(_) => {
								env.set_is_applied(Some(false));
							}
						}
					}
				}
			}
		}
		self.save_to_file(app)
	}

	pub fn check<R: tauri::Runtime>(&mut self, app: tauri::AppHandle<R>) -> anyhow::Result<()> {
		let type_enum = self.get_scope_enum();
		let manager = get_environment_vars_manager(&type_enum);
		let system_envs = manager
			.inner()
			.read_envs()
			.expect("get system environment variables failed!");

		info!("check config before: {:#?}", self);
		if let Some(groups) = &mut self.groups {
			for group in groups.iter_mut() {
				if let Some(envs) = group.envs.as_mut() {
					for env in envs.iter_mut() {
						let key = env.get_key();
						let value = env.get_value();

						if let Some(system_env_value) = system_envs.get(key) {
							if system_env_value.eq(value) {
								env.set_is_same(Some(true));
								env.set_current_value(Some(system_env_value.to_string()));
							} else {
								match manager.inner().set(key, value) {
									Ok(_) => {
										env.set_is_applied(Some(true));
									}
									Err(_) => {
										env.set_is_applied(Some(false));
									}
								}
							}
						} else {
							env.set_is_applied(Some(false));
						}
					}
				}
			}
		}
		info!("check config after: {:#?}", self);
		self.save_to_file(app)
	}
}
