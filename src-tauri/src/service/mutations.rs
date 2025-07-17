// 负责“写”操作（增、改、删），即对数据库的数据进行变更。这里会用到 ActiveModel，通常是业务层的写接口。
use sea_orm::*;

use crate::entity::{env_config, environment_variable, variable_group, variable_group_mapping};

pub struct MutationsService;

impl MutationsService {
	pub async fn create_env_config(
		db: &DbConn,
		env_config: env_config::ActiveModel,
	) -> Result<String, DbErr> {
		match env_config.insert(db).await {
			Ok(res) => Ok(res.id),
			Err(e) => Err(e),
		}
	}

	pub async fn update_env_config(
		db: &DbConn,
		env_config: env_config::ActiveModel,
	) -> Result<(), DbErr> {
		let id = env_config.clone().id.unwrap();
		env_config::Entity::find_by_id(id.clone())
			.one(db)
			.await?
			.ok_or(DbErr::RecordNotFound(format!(
				"没有找到 {} 环境变量配置",
				id
			)))?;

		match env_config.update(db).await {
			Ok(_) => Ok(()),
			Err(e) => Err(e),
		}
	}

	pub async fn delete_env_config(db: &DbConn, id: String) -> Result<(), DbErr> {
		let model = env_config::Entity::find_by_id(id.clone())
			.one(db)
			.await?
			.ok_or(DbErr::RecordNotFound(format!(
				"没有找到 {} 环境变量配置",
				id
			)))?;
		let active_model = model.into_active_model();

		match active_model.delete(db).await {
			Ok(_) => Ok(()),
			Err(e) => Err(e),
		}
	}

	// environment_variable
	pub async fn create_environment_variable(
		db: &DbConn,
		model: environment_variable::ActiveModel,
	) -> Result<String, DbErr> {
		match model.insert(db).await {
			Ok(res) => Ok(res.id),
			Err(e) => Err(e),
		}
	}

	pub async fn update_environment_variable(
		db: &DbConn,
		model: environment_variable::ActiveModel,
	) -> Result<(), DbErr> {
		let id = model.clone().id.unwrap();
		environment_variable::Entity::find_by_id(id.clone())
			.one(db)
			.await?
			.ok_or(DbErr::RecordNotFound(format!("没有找到 {} 环境变量", id)))?;

		match model.update(db).await {
			Ok(_) => Ok(()),
			Err(e) => Err(e),
		}
	}

	pub async fn delete_environment_variable(db: &DbConn, id: String) -> Result<(), DbErr> {
		let found = environment_variable::Entity::find_by_id(id.clone())
			.one(db)
			.await?
			.ok_or(DbErr::RecordNotFound(format!("没有找到 {} 环境变量", id)))?;
		let active_model = found.into_active_model();

		match active_model.delete(db).await {
			Ok(_) => Ok(()),
			Err(e) => Err(e),
		}
	}

	// variable_group
	pub async fn create_variable_group(
		db: &DbConn,
		model: variable_group::ActiveModel,
	) -> Result<String, DbErr> {
		match model.insert(db).await {
			Ok(res) => Ok(res.id),
			Err(e) => Err(e),
		}
	}

	pub async fn update_variable_group(
		db: &DbConn,
		model: variable_group::ActiveModel,
	) -> Result<(), DbErr> {
		let id = model.clone().id.unwrap();
		variable_group::Entity::find_by_id(id.clone())
			.one(db)
			.await?
			.ok_or(DbErr::RecordNotFound(format!(
				"没有找到 {} 分组",
				id.clone()
			)))?;

		match model.update(db).await {
			Ok(_) => Ok(()),
			Err(e) => Err(e),
		}
	}

	pub async fn delete_variable_group(db: &DbConn, id: String) -> Result<(), DbErr> {
		let found = variable_group::Entity::find_by_id(id.clone())
			.one(db)
			.await?
			.ok_or(DbErr::RecordNotFound(format!("没有找到 {} 分组", id)))?;
		let active_model = found.into_active_model();

		match active_model.delete(db).await {
			Ok(_) => Ok(()),
			Err(e) => Err(e),
		}
	}

	// variable_group_mapping
	pub async fn create_variable_group_mapping(
		db: &DbConn,
		model: variable_group_mapping::ActiveModel,
	) -> Result<(), DbErr> {
		match model.insert(db).await {
			Ok(_) => Ok(()),
			Err(e) => Err(e),
		}
	}

	pub async fn update_variable_group_mapping(
		db: &DbConn,
		model: variable_group_mapping::ActiveModel,
	) -> Result<(), DbErr> {
		let group_id = model.clone().group_id.unwrap();
		let variable_id = model.clone().variable_id.unwrap();
		variable_group_mapping::Entity::find()
			.filter(
				sea_orm::Condition::all()
					.add(variable_group_mapping::Column::GroupId.eq(&group_id))
					.add(variable_group_mapping::Column::VariableId.eq(&variable_id)),
			)
			.one(db)
			.await?
			.ok_or(DbErr::RecordNotFound(format!(
				"没有找到 group_id={} variable_id={} 的分组映射",
				&group_id, &variable_id
			)))?;

		match model.update(db).await {
			Ok(_) => Ok(()),
			Err(e) => Err(e),
		}
	}

	pub async fn delete_variable_group_mapping(
		db: &DbConn,
		group_id: String,
		variable_id: String,
	) -> Result<(), DbErr> {
		let found = variable_group_mapping::Entity::find()
			.filter(
				sea_orm::Condition::all()
					.add(variable_group_mapping::Column::GroupId.eq(group_id.clone()))
					.add(variable_group_mapping::Column::VariableId.eq(variable_id.clone())),
			)
			.one(db)
			.await?
			.ok_or(DbErr::RecordNotFound(format!(
				"没有找到 group_id={} variable_id={} 的分组映射",
				group_id, variable_id
			)))?;
		let active_model = found.into_active_model();

		match active_model.delete(db).await {
			Ok(_) => Ok(()),
			Err(e) => Err(e),
		}
	}
}
