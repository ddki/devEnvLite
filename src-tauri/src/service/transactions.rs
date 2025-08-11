use sea_orm::*;

use crate::{
	entity::{env_config, environment_variable, variable_group, variable_group_mapping},
	model::{EnvConfig, EnvironmentVariable},
};

// 负责复杂的事务操作，比如多表联合写入、批量操作等。用于保证数据一致性和原子性
pub struct TransactionService;

impl TransactionService {
	pub async fn create_env_config(
		db: &DbConn,
		config: EnvConfig,
	) -> Result<String, TransactionError<DbErr>> {
		db.transaction::<_, String, DbErr>(|txn| {
			Box::pin(async move {
				let config_id = ulid::Ulid::new().to_string();
				env_config::ActiveModel {
					id: Set(config_id.clone()),
					name: Set(config.name),
					scope: Set(config.scope),
					description: Set(config.description),
					sort: Set(config.sort),
					..Default::default()
				}
				.insert(txn)
				.await?;

				for group_item in config.groups.unwrap_or_default() {
					let group_id = ulid::Ulid::new().to_string();
					let group = variable_group::ActiveModel {
						id: Set(group_id.clone()),
						config_id: Set(config_id.clone()),
						name: Set(group_item.name),
						description: Set(group_item.description),
						sort: Set(group_item.sort),
						..Default::default()
					};

					// 先插入 group 为 group-mapping 提供外键
					variable_group::Entity::insert(group).exec(txn).await?;

					let mut variables = Vec::<environment_variable::ActiveModel>::new();
					let mut variable_group_mapping =
						Vec::<variable_group_mapping::ActiveModel>::new();

					for variable_item in group_item.variables.unwrap_or_default() {
						let variable_id = ulid::Ulid::new().to_string();
						let variable = environment_variable::ActiveModel {
							id: Set(variable_id.clone()),
							key: Set(variable_item.key),
							value: Set(variable_item.value),
							description: Set(variable_item.description),
							..Default::default()
						};
						variables.push(variable);
						let mapping = variable_group_mapping::ActiveModel {
							variable_id: Set(variable_id),
							group_id: Set(group_id.clone()),
							sort: Set(variable_item.sort),
							..Default::default()
						};
						variable_group_mapping.push(mapping);
					}
					if variables.len() > 0 {
						environment_variable::Entity::insert_many(variables)
							.exec(txn)
							.await?;
					}
					if variable_group_mapping.len() > 0 {
						variable_group_mapping::Entity::insert_many(variable_group_mapping)
							.exec(txn)
							.await?;
					}
				}
				Ok(config_id)
			})
		})
		.await
	}

	pub async fn delete_env_config(db: &DbConn, id: String) -> Result<(), TransactionError<DbErr>> {
		db.transaction::<_, (), DbErr>(|txn| {
			Box::pin(async move {
				// 获取配置组
				let groups = variable_group::Entity::find()
					.filter(variable_group::Column::ConfigId.eq(id.clone()))
					.all(txn)
					.await?;
				let group_ids = groups
					.iter()
					.map(|group| group.id.clone())
					.collect::<Vec<String>>();

				// 获取组下的变量
				let variable_group_mappings = variable_group_mapping::Entity::find()
					.filter(variable_group_mapping::Column::GroupId.is_in(group_ids.clone()))
					.all(txn)
					.await?;

				// 删除变量组映射
				variable_group_mapping::Entity::delete_many()
					.filter(variable_group_mapping::Column::GroupId.is_in(group_ids.clone()))
					.exec(txn)
					.await?;

				// 删除变量组
				variable_group::Entity::delete_many()
					.filter(variable_group::Column::Id.is_in(group_ids.clone()))
					.exec(txn)
					.await?;
				
				// 删除变量
				environment_variable::Entity::delete_many()
					.filter(
						environment_variable::Column::Id.is_in(
							variable_group_mappings
								.iter()
								.map(|mapping| mapping.variable_id.clone())
								.collect::<Vec<String>>(),
						),
					)
					.exec(txn)
					.await?;
				

				// 最后删除环境变量配置
				env_config::Entity::delete_by_id(id.clone())
					.exec(txn)
					.await?;
				Ok(())
			})
		})
		.await
	}

	pub async fn create_environment_variable(
		db: &DbConn,
		group_id: String,
		variable: EnvironmentVariable,
	) -> Result<String, TransactionError<DbErr>> {
		db.transaction::<_, String, DbErr>(|txn| {
			Box::pin(async move {
				let variable_id = ulid::Ulid::new().to_string();
				environment_variable::ActiveModel {
					id: Set(variable_id.clone()),
					key: Set(variable.key),
					value: Set(variable.value),
					description: Set(variable.description),
					..Default::default()
				}
				.insert(txn)
				.await?;
				variable_group_mapping::ActiveModel {
					variable_id: Set(variable_id.clone()),
					group_id: Set(group_id),
					sort: Set(variable.sort),
					..Default::default()
				}
				.insert(txn)
				.await?;
				Ok(variable_id.clone())
			})
		})
		.await
	}

	pub async fn update_environment_variable(
		db: &DbConn,
		group_id: String,
		variable: EnvironmentVariable,
	) -> Result<String, TransactionError<DbErr>> {
		db.transaction::<_, String, DbErr>(|txn| {
			let variable_id = variable.id.clone();
			Box::pin(async move {
				environment_variable::ActiveModel {
					id: Set(variable_id.clone()),
					key: Set(variable.key),
					value: Set(variable.value),
					description: Set(variable.description),
					..Default::default()
				}
				.update(txn)
				.await?;

				variable_group_mapping::ActiveModel {
					variable_id: Set(variable_id.clone()),
					group_id: Set(group_id),
					sort: Set(variable.sort),
					..Default::default()
				}
				.update(txn)
				.await?;
				Ok(variable_id.clone())
			})
		})
		.await
	}

	pub async fn delete_environment_variable(
		db: &DbConn,
		group_id: String,
		variable_id: String,
	) -> Result<(), TransactionError<DbErr>> {
		db.transaction::<_, (), DbErr>(|txn| {
			Box::pin(async move {
				environment_variable::Entity::delete_by_id(variable_id.clone())
					.exec(txn)
					.await?;
				variable_group_mapping::Entity::delete_many()
					.filter(variable_group_mapping::Column::VariableId.eq(variable_id.clone()))
					.filter(variable_group_mapping::Column::GroupId.eq(group_id.clone()))
					.exec(txn)
					.await?;
				Ok(())
			})
		})
		.await
	}
}
