use log::log;

use super::{EnvironmentVars, EnvironmentVarsType};

pub struct LinuxEnvironmentVars {
	pub env_type: EnvironmentVarsType,
}

impl LinuxEnvironmentVars {
	pub fn set_env_type(mut self, env_type: &EnvironmentVarsType) -> Self {
		self.env_type = env_type.clone();
		self
	}
}

impl EnvironmentVars for LinuxEnvironmentVars {
	fn read_envs(&self) -> anyhow::Result<std::collections::HashMap<String, String>> {
		todo!()
	}

	fn get_keys(&self) -> anyhow::Result<std::collections::HashSet<String>> {
		todo!()
	}

	fn get_value(&self, key: &str) -> anyhow::Result<String> {
		log!(log::Level::Info, "获取环境变量: {}", key);
		todo!()
	}

	fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
		log!(log::Level::Info, "设置环境变量: {} = {}", key, value);
		todo!()
	}

	fn remove_key(&self, key: &str) -> anyhow::Result<()> {
		log!(log::Level::Info, "删除环境变量: {}", key);
		todo!()
	}

	fn remove_keys(&self, keys: Vec<String>) -> anyhow::Result<()> {
		log!(log::Level::Info, "删除环境变量键: {:?}", keys);
		todo!()
	}

	fn collate(&self, keys: Vec<String>) -> anyhow::Result<()> {
		log!(log::Level::Info, "合并环境变量键: {:?}", keys);
		todo!()
	}

	fn sort_value(&self, value: &str) -> anyhow::Result<String> {
		log!(log::Level::Info, "排序环境变量值: {}", value);
		todo!()
	}
}
