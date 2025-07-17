use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct EnvironmentVariable {
	pub id: String,
	pub key: String,
	pub value: String,
	pub description: Option<String>,
	pub sort: Option<i32>,
}

impl From<crate::entity::environment_variable::Model> for EnvironmentVariable {
	fn from(model: crate::entity::environment_variable::Model) -> Self {
		EnvironmentVariable {
			id: model.id,
			key: model.key,
			value: model.value,
			description: model.description,
			// 这里先给默认值，调用from之后，手动设置sort
			sort: Some(1),
		}
	}
}

impl Into<crate::entity::environment_variable::ActiveModel> for EnvironmentVariable {
	fn into(self) -> crate::entity::environment_variable::ActiveModel {
		let id = if self.id.is_empty() { ulid::Ulid::new().to_string() } else { self.id };
		crate::entity::environment_variable::ActiveModel {
			id: sea_orm::ActiveValue::Set(id),
			key: sea_orm::ActiveValue::Set(self.key),
			value: sea_orm::ActiveValue::Set(self.value),
			description: sea_orm::ActiveValue::Set(self.description),
		}
	}
}
