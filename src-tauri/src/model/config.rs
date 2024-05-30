use std::path::PathBuf;

use anyhow::{Error, Result};
use log::{debug, error};
use lombok::{Getter, Setter};
use serde::{Deserialize, Serialize};

use crate::{
	environment_vars::{get_environment_vars_manager, EnvironmentVars, EnvironmentVarsType},
	model::settings,
};

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

	pub fn get_dir<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<PathBuf> {
		let settings = settings::Settings::load_from_store(app.clone())?;
		let dir = PathBuf::from(settings.get_data_dir())
			.join("config");
		debug!("config dir: {:#?}", dir);
		Ok(dir)
	}

	pub fn get_path<R: tauri::Runtime>(id: &str, app: tauri::AppHandle<R>) -> Result<PathBuf> {
		let path = Self::get_dir(app.clone())?
			.join(format!("{}.json", id));
		debug!("config file path: {:#?}", path);
		Ok(path)
	}

	pub fn load_from_file<R: tauri::Runtime>(id: &str, app: tauri::AppHandle<R>) -> Result<Self> {
		let path = Self::get_path(id, app.clone())?;
		if !path.exists() {
			return Err(Error::msg("not found config"));
		}
		match std::fs::read_to_string(path) {
			Ok(content) => {
				let config = serde_json::from_str(&content).expect("config file format error");
				debug!("load config from file config: {:#?}", config);
				Ok(config)
			}
			Err(e) => {
				error!("load config from file error: {:#?}", e);
				Err(Error::msg("load config from file error"))
			}
		}
	}

	pub fn get_group<R: tauri::Runtime>(
		config_id: &str,
		group_id: &str,
		app: tauri::AppHandle<R>,
	) -> Result<GroupInfo> {
		let config = ConfigInfo::load_from_file(config_id, app)?;
		if let Some(groups) = config.groups {
			for group in groups.iter() {
				if group.get_id().eq(group_id) {
					return Ok(group.clone());
				}
			}
		}
		Err(Error::msg("not found group from config"))
	}

	pub fn get_group_env<R: tauri::Runtime>(
		config_id: &str,
		group_id: &str,
		env_key: &str,
		app: tauri::AppHandle<R>,
	) -> Result<EnvInfo> {
		let group = ConfigInfo::get_group(config_id, group_id, app)?;
		if let Some(envs) = group.get_envs() {
			for env in envs.iter() {
				if env.get_key().eq(env_key) {
					return Ok(env.clone());
				}
			}
		}
		Err(Error::msg("not found env from config"))
	}

	/// 更新组信息
	/// 返回是否更新成功
	pub fn update_group(&mut self, new_group: GroupInfo) -> Result<bool> {
		if let Some(groups) = &mut self.groups {
			for group in groups.iter_mut() {
				if group.get_id().eq(new_group.get_id()) {
					*group = new_group;
					return Ok(true);
				}
			}
		} else {
			self.groups = Some(vec![new_group]);
			return Ok(true);
		}
		Ok(false)
	}

	pub fn save_to_file<R: tauri::Runtime>(&mut self, app: tauri::AppHandle<R>) -> Result<()> {
		let path = Self::get_path(self.get_id(), app.clone())?;
		if !path.exists() {
				std::fs::create_dir_all(path.parent().unwrap())?;
		}
		if let Some(groups) = &mut self.groups {
			for group in groups.iter_mut() {
				if let Some(envs) = group.get_envs().clone() {
					let applied_count = envs
						.iter()
						.filter(|x| match x.get_is_applied() {
							Some(is_applied) => *is_applied,
							None => false,
						})
						.count() as u64;
					group.set_env_applied_count(applied_count);
					let not_applied_count = (envs.len() as u64) - applied_count;
					group.set_env_not_applied_count(not_applied_count);
				}
			}
		}

		debug!("save config from file config: {:#?}, path: {:#?}", self, path);
		let file = std::fs::File::create(path)?;
		serde_json::to_writer(file, self).expect("save config failed");

		Ok(())
	}

	pub fn remove_file<R: tauri::Runtime>(id: &str, app: tauri::AppHandle<R>) -> Result<()> {
		let path = Self::get_path(id, app.clone())?;
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

		debug!("check config before: {:#?}", self);
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
		debug!("check config after: {:#?}", self);
		self.save_to_file(app)
	}
}
