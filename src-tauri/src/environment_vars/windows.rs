use std::collections::HashSet;

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
	fn get_keys(&self) -> Option<HashSet<String>> {
		println!(
			"{:?}, {}",
			self.env_type,
			self.env_type == EnvironmentVarsType::SYSTEM
		);
		if self.env_type == EnvironmentVarsType::SYSTEM {
			// 系统环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
			let cur_ver = hklm.open_subkey(SYSTEM_SUB_HKEY).unwrap();
			let mut keys = HashSet::new();
			for (name, value) in cur_ver.enum_values().map(|x| x.unwrap()) {
				println!("{} = {:?}", name, value);
				keys.insert(name);
			}
			Some(keys)
		} else {
			// 用户环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_CURRENT_USER);
			let cur_ver = hklm.open_subkey(USER_SUB_HKEY).unwrap();
			let mut keys = HashSet::new();
			for (name, value) in cur_ver.enum_values().map(|x| x.unwrap()) {
				println!("{} = {:?}", name, value);
				keys.insert(name);
			}
			Some(keys)
		}
	}

	fn get_value(&self, key: &str) -> Option<String> {
		if self.env_type == EnvironmentVarsType::SYSTEM {
			// 系统环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
			let cur_ver = hklm.open_subkey(SYSTEM_SUB_HKEY).unwrap();
			Some(cur_ver.get_value(key).unwrap())
		} else {
			// 用户环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_CURRENT_USER);
			let cur_ver = hklm.open_subkey(USER_SUB_HKEY).unwrap();
			Some(cur_ver.get_value(key).unwrap())
		}
	}

	fn set(&self, key: &str, value: &str) -> bool {
		if self.env_type == EnvironmentVarsType::SYSTEM {
			// 系统环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
			let cur_ver = hklm.open_subkey(SYSTEM_SUB_HKEY).unwrap();
			match cur_ver.set_value(key, &value) {
				Ok(_) => true,
				Err(_) => false,
			}
		} else {
			// 用户环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_CURRENT_USER);
			let cur_ver = hklm.open_subkey(USER_SUB_HKEY).unwrap();
			match cur_ver.set_value(key, &value) {
				Ok(_) => true,
				Err(_) => false,
			}
		}
	}

	fn remove_key(&self, key: &str) -> bool {
		if self.env_type == EnvironmentVarsType::SYSTEM {
			// 系统环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
			let cur_ver = hklm.open_subkey(SYSTEM_SUB_HKEY).unwrap();
			match cur_ver.delete_subkey(key) {
				Ok(_) => true,
				Err(_) => false,
			}
		} else {
			// 用户环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_CURRENT_USER);
			let cur_ver = hklm.open_subkey(USER_SUB_HKEY).unwrap();
			match cur_ver.delete_subkey(key) {
				Ok(_) => true,
				Err(_) => false,
			}
		}
	}

	fn remove_keys(&self, keys: Vec<String>) -> bool {
		let mut count = 0;
		for key in &keys {
			if self.remove_key(&key) {
				count += 1;
			}
		}
		count == keys.len()
	}

	fn collate(&self, keys: Vec<String>) {
		if self.env_type == EnvironmentVarsType::SYSTEM {
			// 系统环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
			let cur_ver = hklm.open_subkey(SYSTEM_SUB_HKEY).unwrap();
			for (key, value) in cur_ver
				.enum_values()
				.map(|x| x.unwrap())
				.filter(|x| keys.contains(&x.0))
			{
				let _ = cur_ver.set_value(key, &self.sort_value(&value.to_string()));
			}
		} else {
			// 用户环境变量
			// 打开注册表
			let hklm = RegKey::predef(HKEY_CURRENT_USER);
			let cur_ver = hklm.open_subkey(USER_SUB_HKEY).unwrap();
			for (key, value) in cur_ver
				.enum_values()
				.map(|x| x.unwrap())
				.filter(|x| keys.contains(&x.0))
			{
				let _ = cur_ver.set_value(key, &self.sort_value(&value.to_string()));
			}
		}
	}

	fn sort_value(&self, value: &str) -> String {
		let mut values: Vec<&str> = value.split(VALUE_SPLITOR).collect();
		values.sort();
		values.join(VALUE_SPLITOR)
	}
}
