use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use crate::{entity::env_config, model::VariableGroup};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct EnvConfig {
	pub id: String,
	pub name: String,
	pub scope: String,
	pub description: Option<String>,
	pub is_active: bool,
	pub sort: Option<i32>,
	pub groups: Option<Vec<VariableGroup>>,
}

impl From<env_config::Model> for EnvConfig {
	fn from(model: env_config::Model) -> Self {
		EnvConfig {
			id: model.id,
			name: model.name,
			scope: model.scope,
			description: model.description,
			is_active: model.is_active,
			sort: model.sort,
			groups: Some(vec![]),
		}
	}
}

impl Into<env_config::ActiveModel> for EnvConfig {
	fn into(self) -> env_config::ActiveModel {
		let id = if self.id.is_empty() { ulid::Ulid::new().to_string() } else { self.id };
		env_config::ActiveModel {
			id: Set(id),
			name: Set(self.name),
			scope: Set(self.scope),
			description: Set(self.description),
			is_active: Set(self.is_active),
			sort: Set(self.sort),
		}
	}
}
