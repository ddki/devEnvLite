pub mod linux;
pub mod windows;

use std::collections::HashSet;

pub trait EnvironmentVars {
	fn get_keys(&self) -> Option<HashSet<String>>;
	fn get_value(&self, key: &str) -> Option<String>;
	fn set(&self, key: &str, value: &str) -> bool;
	fn remove_key(&self, key: &str) -> bool;
	fn remove_keys(&self, keys: Vec<String>) -> bool;
	fn collate(&self, keys: Vec<String>);
	fn sort_value(&self, value: &str) -> String;
}

#[cfg(target_os = "windows")]
type EnvironmentVarsImpl = windows::WindowEnvironmentVars;

#[cfg(target_os = "linux")]
type EnvironmentVarsImpl = linux::LinuxEnvironmentVars;

pub fn get_environment_vars_manager(
	env_type: &EnvironmentVarsType,
) -> EnvironmentVarsManager<EnvironmentVarsImpl> {
	EnvironmentVarsManager::new(EnvironmentVarsImpl {
		env_type: env_type.clone(),
	})
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
	pub fn inner(&self) -> &T {
		&self.inner
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnvironmentVarsType {
	USER,
	SYSTEM,
}

impl std::fmt::Display for EnvironmentVarsType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			EnvironmentVarsType::SYSTEM => write!(f, "USER"),
			EnvironmentVarsType::USER => write!(f, "SYSTEM"),
		}
	}
}
