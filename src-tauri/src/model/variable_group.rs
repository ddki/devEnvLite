use serde::{Deserialize, Serialize};

use crate::model::EnvironmentVariable;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct VariableGroup {
	pub id: String,
	pub config_id: String,
	pub name: String,
	pub description: Option<String>,
	pub sort: Option<i32>,
	pub variables: Option<Vec<EnvironmentVariable>>,
}

impl From<crate::entity::variable_group::Model> for VariableGroup {
	fn from(model: crate::entity::variable_group::Model) -> Self {
		VariableGroup {
			id: model.id,
			config_id: model.config_id,
			name: model.name,
			description: model.description,
			sort: model.sort,
			variables: Some(vec![]),
		}
	}
}

impl Into<crate::entity::variable_group::ActiveModel> for VariableGroup {
	fn into(self) -> crate::entity::variable_group::ActiveModel {
		let id = if self.id.is_empty() { ulid::Ulid::new().to_string() } else { self.id };
		crate::entity::variable_group::ActiveModel {
			id: sea_orm::ActiveValue::Set(id),
			config_id: sea_orm::ActiveValue::Set(self.config_id),
			name: sea_orm::ActiveValue::Set(self.name),
			description: sea_orm::ActiveValue::Set(self.description),
			sort: sea_orm::ActiveValue::Set(self.sort),
		}
	}
}
