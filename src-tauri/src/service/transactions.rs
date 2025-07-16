use sea_orm::*;

use crate::{
	entity::{env_config, environment_variable, variable_group, variable_group_mapping},
	model::EnvConfig,
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
				env_config::ActiveModel {
					id: Set(ulid::Ulid::new().to_string()),
					name: Set(config.name),
					scope: Set(config.scope),
					description: Set(config.description),
					sort: Set(config.sort),
					..Default::default()
				}
				.insert(txn)
				.await?;

				let mut groups = Vec::<variable_group::ActiveModel>::new();

				for group_item in config.groups.unwrap_or_default() {
					let group = variable_group::ActiveModel {
						id: Set(ulid::Ulid::new().to_string()),
						config_id: Set(config.id.clone()),
						name: Set(group_item.name),
						description: Set(group_item.description),
						sort: Set(group_item.sort),
						..Default::default()
					};
					let group_id = group.id.clone().unwrap();
					groups.push(group);

					let mut variables = Vec::<environment_variable::ActiveModel>::new();
					let mut variable_group_mapping =
						Vec::<variable_group_mapping::ActiveModel>::new();

					for variable_item in group_item.variables.unwrap_or_default() {
						let variable = environment_variable::ActiveModel {
							id: Set(ulid::Ulid::new().to_string()),
							key: Set(variable_item.key),
							value: Set(variable_item.value),
							description: Set(variable_item.description),
							..Default::default()
						};
						let variable_id = variable.id.clone().unwrap();
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

				if groups.len() > 0 {
					variable_group::Entity::insert_many(groups)
						.exec(txn)
						.await?;
				}
				Ok(config.id)
			})
		})
		.await
	}
}
