use serde::{Deserialize, Serialize};

use crate::{entity::env_config, model::VariableGroup};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct EnvConfig {
	pub id: String,
	pub name: String,
	pub scope: String,
	pub description: Option<String>,
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
            sort: model.sort,
            groups: Some(vec![]),
        }
    }
}