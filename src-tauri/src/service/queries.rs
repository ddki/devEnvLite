use sea_orm::*;

use crate::{
	entity::{env_config, environment_variable, variable_group, variable_group_mapping},
	model::{EnvConfig, EnvironmentVariable, VariableGroup},
};

// 负责“读”操作（查），即从数据库获取数据。这里会用到 Entity，通常是业务层的查询接口
pub struct QueriesService;

impl QueriesService {
	pub async fn list_env_configs(db: &DbConn) -> Result<Vec<env_config::Model>, DbErr> {
		env_config::Entity::find()
			.order_by_desc(env_config::Column::IsActive)
			.order_by_asc(env_config::Column::Sort)
			.all(db)
			.await
	}

	pub async fn list_env_configs_with_group_active(db: &DbConn) -> Result<Vec<EnvConfig>, DbErr> {
		let active_configs = env_config::Entity::find()
			.filter(env_config::Column::IsActive.eq(true))
			.order_by_desc(env_config::Column::IsActive)
			.order_by_asc(env_config::Column::Sort)
			.all(db)
			.await?;
		let result = active_configs
			.into_iter()
			.map(|config| async { Self::get_env_config_with_groups(db, config.id).await })
			.collect::<futures::future::JoinAll<_>>()
			.await
			.into_iter()
			.collect::<Result<Vec<Option<EnvConfig>>, DbErr>>()?
			.into_iter()
			.flatten()
			.collect::<Vec<EnvConfig>>();
		Ok(result)
	}

	pub async fn list_env_configs_by_name(
		db: &DbConn,
		exclude_config_id: Option<String>,
		name: String,
	) -> Result<Vec<env_config::Model>, DbErr> {
		let mut conditions = Condition::all();
		if let Some(config_id) = exclude_config_id {
			conditions = conditions.add(env_config::Column::Id.ne(config_id));
		}
		env_config::Entity::find()
			.filter(conditions)
			.filter(env_config::Column::Name.eq(name))
			.all(db)
			.await
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
				let group = Self::build_variable_group_with_variables(db, g).await?;
				vg_list.push(group);
			}
			let mut config = EnvConfig::from(config);
			config.groups = Some(vg_list);
			Ok(Some(config))
		} else {
			Ok(None)
		}
	}

	pub async fn get_variable_group_with_variables(
		db: &DbConn,
		id: String,
	) -> Result<Option<VariableGroup>, DbErr> {
		let group = Self::get_variable_group(db, id.clone()).await?;
		match group {
			Some(g) => {
				let group = Self::build_variable_group_with_variables(db, g).await?;
				Ok(Some(group))
			}
			None => Ok(None),
		}
	}

	async fn build_variable_group_with_variables(
		db: &DbConn,
		group: variable_group::Model,
	) -> Result<VariableGroup, DbErr> {
		let variables = Self::list_environment_variables(db, group.id.clone()).await?;
		let variable_group_mappings = variable_group_mapping::Entity::find()
			.filter(variable_group_mapping::Column::GroupId.eq(group.id.clone()))
			.all(db)
			.await?;
		let mut group = VariableGroup::from(group);
		group.variables = Some(
			variables
				.into_iter()
				.map(EnvironmentVariable::from)
				.map(|mut var| {
					variable_group_mappings
						.iter()
						.find(|m| m.variable_id == var.id)
						.map(|m| {
							var.sort = m.sort;
						});
					var
				})
				.collect(),
		);
		Ok(group)
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

	pub async fn list_variable_groups_with_variables(
		db: &DbConn,
		config_id: String,
	) -> Result<Vec<VariableGroup>, DbErr> {
		let groups = variable_group::Entity::find()
			.filter(variable_group::Column::ConfigId.eq(config_id))
			.all(db)
			.await?;

		let mut vg_list = Vec::new();
		for group in groups.iter() {
			let group = Self::build_variable_group_with_variables(db, group.clone()).await?;
			vg_list.push(group);
		}
		Ok(vg_list)
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
