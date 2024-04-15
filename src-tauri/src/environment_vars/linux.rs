use std::collections::HashSet;

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
	fn get_keys(&self) -> Option<HashSet<String>> {
		todo!()
	}

	fn get_value(&self, key: &str) -> Option<String> {
		todo!()
	}

	fn set(&self, key: &str, value: &str) -> bool {
		todo!()
	}

	fn remove_key(&self, key: &str) -> bool {
		todo!()
	}

	fn remove_keys(&self, keys: Vec<String>) -> bool {
		todo!()
	}

	fn collate(&self, keys: Vec<String>) {
		todo!()
	}

	fn sort_value(&self, value: &str) -> String {
		todo!()
	}
}
