use sea_orm::*;

use crate::{
	entity::{env_config, environment_variable, variable_group, variable_group_mapping},
	model::{EnvConfig, EnvironmentVariable, VariableGroup},
};

// 负责“读”操作（查），即从数据库获取数据。这里会用到 Entity，通常是业务层的查询接口
pub struct QueriesService;

impl QueriesService {
	pub async fn list_env_configs(db: &DbConn) -> Result<Vec<env_config::Model>, DbErr> {
		env_config::Entity::find().all(db).await
	}

	pub async fn get_env_config(
		db: &DbConn,
		id: String,
	) -> Result<Option<env_config::Model>, DbErr> {
		env_config::Entity::find_by_id(id).one(db).await
	}

	pub async fn get_env_config_with_groups(
		db: &DbConn,
		id: String,
	) -> Result<Option<EnvConfig>, DbErr> {
		let env_configs = env_config::Entity::find_by_id(id)
			.find_with_related(variable_group::Entity)
			.all(db)
			.await?;
		if let Some((config, groups)) = env_configs.into_iter().next() {
			let mut vg_list = Vec::new();
			for g in groups {
				let variables = Self::list_environment_variables(db, g.id.clone()).await?;
				let variable_group_mappings = variable_group_mapping::Entity::find()
					.filter(variable_group_mapping::Column::GroupId.eq(g.id.clone()))
					.all(db)
					.await?;
				let mut group = VariableGroup::from(g);
				group.variables = Some(
					variables
						.into_iter()
						.map(EnvironmentVariable::from)
						.map(|mut var| {
							variable_group_mappings.iter().find(|m| m.variable_id == var.id).map(|m| {
								var.sort = m.sort;
							});
							var
						})
						.collect(),
				);
				vg_list.push(group);
			}
			Ok(Some(EnvConfig::from(config)))
		} else {
			Ok(None)
		}
	}

	pub async fn list_variable_groups(
		db: &DbConn,
		config_id: String,
	) -> Result<Vec<variable_group::Model>, DbErr> {
		variable_group::Entity::find()
			.filter(variable_group::Column::ConfigId.eq(config_id))
			.all(db)
			.await
	}

	pub async fn get_variable_group(
		db: &DbConn,
		id: String,
	) -> Result<Option<variable_group::Model>, DbErr> {
		variable_group::Entity::find_by_id(id).one(db).await
	}

	pub async fn list_environment_variables(
		db: &DbConn,
		group_id: String,
	) -> Result<Vec<environment_variable::Model>, DbErr> {
		let query = environment_variable::Entity::find()
			.find_with_related(variable_group_mapping::Entity)
			.filter(variable_group_mapping::Column::GroupId.eq(group_id));
		let variables_with_groups = query.all(db).await?;
		Ok(variables_with_groups
			.into_iter()
			.map(|(variable, _)| variable)
			.collect())
	}

	pub async fn get_environment_variable(
		db: &DbConn,
		id: String,
	) -> Result<Option<environment_variable::Model>, DbErr> {
		environment_variable::Entity::find_by_id(id).one(db).await
	}
}
