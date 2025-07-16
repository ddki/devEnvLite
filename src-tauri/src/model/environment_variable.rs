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
            sort: Some(1)
        }
    }
    
}