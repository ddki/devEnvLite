use std::collections::{HashMap, HashSet};

use anyhow::Ok;

use winreg::{
	enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
	RegKey,
};

use super::{EnvironmentVars, EnvironmentVarsType};

const SYSTEM_SUB_HKEY: &str = "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment";
const USER_SUB_HKEY: &str = "Environment";
const VALUE_SPLITOR: &str = ";";

pub struct WindowEnvironmentVars {
	pub env_type: EnvironmentVarsType,
}

impl WindowEnvironmentVars {
	pub fn set_env_type(mut self, env_type: &EnvironmentVarsType) -> Self {
		self.env_type = env_type.clone();
		self
	}
}

impl EnvironmentVars for WindowEnvironmentVars {
	fn read_envs(&self) -> anyhow::Result<std::collections::HashMap<String, String>> {
		println!("{:?}", self.env_type);
		if self.env_type == EnvironmentVarsType::SYSTEM {
			// 系统环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
			let cur_ver = hklm.open_subkey(SYSTEM_SUB_HKEY).unwrap();
			let mut keys = HashMap::new();
			for (name, value) in cur_ver.enum_values().map(|x| x.unwrap()) {
				// println!("{} = {:?}", name, value);
				keys.insert(name, value.to_string());
			}
			let mut key_list: Vec<String> = keys.keys().cloned().collect();
			key_list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

			let mut return_keys = HashMap::new();
			for item_key in key_list {
				return_keys.insert(item_key.clone(), keys[&item_key].clone());
			}
			Ok(return_keys)
		} else {
			// 用户环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_CURRENT_USER);
			let cur_ver = hklm.open_subkey(USER_SUB_HKEY).unwrap();
			let mut keys = HashMap::new();
			for (name, value) in cur_ver.enum_values().map(|x| x.unwrap()) {
				// println!("{} = {:?}", name, value);
				keys.insert(name, value.to_string());
			}
			let mut key_list: Vec<String> = keys.keys().cloned().collect();
			key_list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

			let mut return_keys = HashMap::new();
			for item_key in key_list {
				return_keys.insert(item_key.clone(), keys[&item_key].clone());
			}
			Ok(return_keys)
		}
	}
	fn get_keys(&self) -> anyhow::Result<HashSet<String>> {
		println!("{:?}", self.env_type);
		if self.env_type == EnvironmentVarsType::SYSTEM {
			// 系统环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
			let cur_ver = hklm.open_subkey(SYSTEM_SUB_HKEY).unwrap();
			let mut keys = HashSet::new();
			for (name, _value) in cur_ver.enum_values().map(|x| x.unwrap()) {
				// println!("{} = {:?}", name, value);
				keys.insert(name);
			}
			Ok(keys)
		} else {
			// 用户环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_CURRENT_USER);
			let cur_ver = hklm.open_subkey(USER_SUB_HKEY).unwrap();
			let mut keys = HashSet::new();
			for (name, _value) in cur_ver.enum_values().map(|x| x.unwrap()) {
				// println!("{} = {:?}", name, value);
				keys.insert(name);
			}
			Ok(keys)
		}
	}

	fn get_value(&self, key: &str) -> anyhow::Result<String> {
		if self.env_type == EnvironmentVarsType::SYSTEM {
			// 系统环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
			let cur_ver = hklm.open_subkey(SYSTEM_SUB_HKEY)?;
			Ok(cur_ver.get_value(key)?)
		} else {
			// 用户环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_CURRENT_USER);
			let cur_ver = hklm.open_subkey(USER_SUB_HKEY)?;
			Ok(cur_ver.get_value(key)?)
		}
	}

	fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
		if self.env_type == EnvironmentVarsType::SYSTEM {
			// 系统环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
			let cur_ver = hklm.open_subkey(SYSTEM_SUB_HKEY)?;
			cur_ver.set_value(key, &value)?;
			Ok(())
		} else {
			// 用户环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_CURRENT_USER);
			let cur_ver = hklm.open_subkey(USER_SUB_HKEY)?;
			cur_ver.set_value(key, &value)?;
			Ok(())
		}
	}

	fn remove_key(&self, key: &str) -> anyhow::Result<()> {
		if self.env_type == EnvironmentVarsType::SYSTEM {
			// 系统环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
			let cur_ver = hklm.open_subkey(SYSTEM_SUB_HKEY)?;
			cur_ver.delete_subkey(key)?;
			Ok(())
		} else {
			// 用户环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_CURRENT_USER);
			let cur_ver = hklm.open_subkey(USER_SUB_HKEY)?;
			cur_ver.delete_subkey(key)?;
			Ok(())
		}
	}

	fn remove_keys(&self, keys: Vec<String>) -> anyhow::Result<()> {
		for key in &keys {
			self.remove_key(&key)?;
		}
		Ok(())
	}

	fn collate(&self, keys: Vec<String>) -> anyhow::Result<()> {
		if self.env_type == EnvironmentVarsType::SYSTEM {
			// 系统环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
			let cur_ver = hklm.open_subkey(SYSTEM_SUB_HKEY)?;
			for (key, value) in cur_ver
				.enum_values()
				.map(|x| x.unwrap())
				.filter(|x| keys.contains(&x.0))
			{
				let sort_value = self.sort_value(&value.to_string());
				println!(
					"collate: sort_value = {:?}",
					sort_value.as_ref().unwrap().clone()
				);
				let _ = cur_ver.set_value(key, &sort_value.unwrap().clone());
			}
			Ok(())
		} else {
			// 用户环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_CURRENT_USER);
			let cur_ver = hklm.open_subkey(USER_SUB_HKEY)?;
			for (key, value) in cur_ver
				.enum_values()
				.map(|x| x.unwrap())
				.filter(|x| keys.contains(&x.0))
			{
				let sort_value = &self.sort_value(&value.to_string());
				println!(
					"collate: sort_value = {:?}",
					sort_value.as_ref().unwrap().clone()
				);
				let _ = cur_ver.set_value(key, &sort_value.as_ref().unwrap().clone());
			}
			Ok(())
		}
	}

	fn sort_value(&self, value: &str) -> anyhow::Result<String> {
		let mut values: Vec<&str> = value.split(VALUE_SPLITOR).filter(|a| a.len() > 0).collect();
		values.sort_by(|a, b| a.to_uppercase().cmp(&b.to_uppercase()));
		let sort_value = values.join(VALUE_SPLITOR);
		Ok(sort_value)
	}
}
