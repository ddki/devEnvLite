// 负责“写”操作（增、改、删），即对数据库的数据进行变更。这里会用到 ActiveModel，通常是业务层的写接口。
use sea_orm::*;

use crate::entity::{env_config, environment_variable, variable_group, variable_group_mapping};

pub struct MutationsService;

impl MutationsService {
	pub async fn create_env_config(
		db: &DbConn,
		env_config: env_config::Model,
	) -> Result<String, DbErr> {
		let active_model = env_config::ActiveModel {
			name: Set(env_config.name),
			scope: Set(env_config.scope),
			description: Set(env_config.description),
			sort: Set(env_config.sort),
			..Default::default()
		};

		match active_model.insert(db).await {
			Ok(res) => Ok(res.id),
			Err(e) => Err(e),
		}
	}

	pub async fn update_env_config(
		db: &DbConn,
		env_config: env_config::Model,
	) -> Result<(), DbErr> {
		let id = env_config.id.clone();
		let model = env_config::Entity::find_by_id(id.clone())
			.one(db)
			.await?
			.ok_or(DbErr::RecordNotFound(format!(
				"没有找到 {} 环境变量配置",
				id
			)))?;
		let mut active_model = model.into_active_model();
		active_model.name = Set(env_config.name);
		active_model.scope = Set(env_config.scope);
		active_model.description = Set(env_config.description);
		active_model.sort = Set(env_config.sort);

		match active_model.update(db).await {
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
        model: environment_variable::Model,
    ) -> Result<String, DbErr> {
        let active_model = environment_variable::ActiveModel {
            key: Set(model.key),
            value: Set(model.value),
            description: Set(model.description),
            ..Default::default()
        };
        match active_model.insert(db).await {
            Ok(res) => Ok(res.id),
            Err(e) => Err(e),
        }
    }

    pub async fn update_environment_variable(
        db: &DbConn,
        model: environment_variable::Model,
    ) -> Result<(), DbErr> {
        let id = model.id.clone();
        let found = environment_variable::Entity::find_by_id(id.clone())
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "没有找到 {} 环境变量", id
            )))?;
        let mut active_model = found.into_active_model();
        active_model.key = Set(model.key);
        active_model.value = Set(model.value);
        active_model.description = Set(model.description);

        match active_model.update(db).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_environment_variable(db: &DbConn, id: String) -> Result<(), DbErr> {
        let found = environment_variable::Entity::find_by_id(id.clone())
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "没有找到 {} 环境变量", id
            )))?;
        let active_model = found.into_active_model();

        match active_model.delete(db).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    // variable_group
    pub async fn create_variable_group(
        db: &DbConn,
        model: variable_group::Model,
    ) -> Result<String, DbErr> {
        let active_model = variable_group::ActiveModel {
            config_id: Set(model.config_id),
            name: Set(model.name),
            description: Set(model.description),
            is_active: Set(model.is_active),
            sort: Set(model.sort),
            ..Default::default()
        };
        match active_model.insert(db).await {
            Ok(res) => Ok(res.id),
            Err(e) => Err(e),
        }
    }

    pub async fn update_variable_group(
        db: &DbConn,
        model: variable_group::Model,
    ) -> Result<(), DbErr> {
        let id = model.id.clone();
        let found = variable_group::Entity::find_by_id(id.clone())
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "没有找到 {} 分组", id
            )))?;
        let mut active_model = found.into_active_model();
        active_model.config_id = Set(model.config_id);
        active_model.name = Set(model.name);
        active_model.description = Set(model.description);
        active_model.is_active = Set(model.is_active);
        active_model.sort = Set(model.sort);

        match active_model.update(db).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_variable_group(db: &DbConn, id: String) -> Result<(), DbErr> {
        let found = variable_group::Entity::find_by_id(id.clone())
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "没有找到 {} 分组", id
            )))?;
        let active_model = found.into_active_model();

        match active_model.delete(db).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    // variable_group_mapping
    pub async fn create_variable_group_mapping(
        db: &DbConn,
        model: variable_group_mapping::Model,
    ) -> Result<(), DbErr> {
        let active_model = variable_group_mapping::ActiveModel {
            group_id: Set(model.group_id),
            variable_id: Set(model.variable_id),
            sort: Set(model.sort),
            ..Default::default()
        };
        match active_model.insert(db).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn update_variable_group_mapping(
        db: &DbConn,
        model: variable_group_mapping::Model,
    ) -> Result<(), DbErr> {
        let found = variable_group_mapping::Entity::find()
            .filter(
                sea_orm::Condition::all()
                    .add(variable_group_mapping::Column::GroupId.eq(model.group_id.clone()))
                    .add(variable_group_mapping::Column::VariableId.eq(model.variable_id.clone()))
            )
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "没有找到 group_id={} variable_id={} 的分组映射", model.group_id, model.variable_id
            )))?;
        let mut active_model = found.into_active_model();
        active_model.sort = Set(model.sort);

        match active_model.update(db).await {
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
                    .add(variable_group_mapping::Column::VariableId.eq(variable_id.clone()))
            )
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "没有找到 group_id={} variable_id={} 的分组映射", group_id, variable_id
            )))?;
        let active_model = found.into_active_model();

        match active_model.delete(db).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
