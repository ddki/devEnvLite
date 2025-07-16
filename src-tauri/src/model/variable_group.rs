use serde::{Deserialize, Serialize};

use crate::model::EnvironmentVariable;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct VariableGroup {
	pub id: String,
	pub config_id: String,
	pub name: String,
	pub description: Option<String>,
	pub is_active: bool,
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
            is_active: model.is_active,
            sort: model.sort,
            variables: Some(vec![]),
        }
    }
}