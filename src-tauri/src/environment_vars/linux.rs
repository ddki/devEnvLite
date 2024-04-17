
use super::{EnvironmentVarsType};

pub struct LinuxEnvironmentVars {
	pub env_type: EnvironmentVarsType,
}

impl LinuxEnvironmentVars {
	pub fn set_env_type(mut self, env_type: &EnvironmentVarsType) -> Self {
		self.env_type = env_type.clone();
		self
	}
}

// impl EnvironmentVars for LinuxEnvironmentVars {
// 	fn get_keys(&self) -> anyhow::Result<HashSet<String>> {
// 		todo!()
// 	}

// 	fn get_value(&self, key: &str) -> anyhow::Result<String> {
// 		todo!()
// 	}

// 	fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
// 		todo!()
// 	}

// 	fn remove_key(&self, key: &str) -> anyhow::Result<()> {
// 		todo!()
// 	}

// 	fn remove_keys(&self, keys: Vec<String>) -> anyhow::Result<()> {
// 		todo!()
// 	}

// 	fn collate(&self, keys: Vec<String>) -> anyhow::Result<()> {
// 		todo!()
// 	}

// 	fn sort_value(&self, value: &str) -> anyhow::Result<String> {
// 		todo!()
// 	}
// }
