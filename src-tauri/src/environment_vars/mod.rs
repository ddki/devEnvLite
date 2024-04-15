pub mod linux;
pub mod windows;

use std::collections::HashSet;

trait EnvironmentVars {
	fn get_keys(&self) -> Option<HashSet<String>>;
	fn get_value(&self, key: &str) -> Option<String>;
	fn set(&self, key: &str, value: &str) -> bool;
	fn remove_key(&self, key: &str) -> bool;
	fn remove_keys(&self, keys: Vec<String>) -> bool;
}


#[cfg(target_os = "windows")]
type EnvironmentVarsImpl = windows::WindowEnvironmentVars;

#[cfg(target_os = "linux")]
type EnvironmentVarsImpl = linux::LinuxEnvironmentVars;

pub fn get_environment_vars_manager() -> EnvironmentVarsManager<EnvironmentVarsImpl> {
	EnvironmentVarsManager::new(EnvironmentVarsImpl { env_type: EnvironmentVarsType::SYSTEM })
}

/// 环境变量管理
/// 环境变量的增删改查
pub struct EnvironmentVarsManager<T> {
	inner: T,
}

impl<T> EnvironmentVarsManager<T> {
	pub fn new(inner: T) -> Self {
		EnvironmentVarsManager { inner }
	}
}

#[derive(Debug, Clone)]
pub enum EnvironmentVarsType {
	USER,
	SYSTEM
}
