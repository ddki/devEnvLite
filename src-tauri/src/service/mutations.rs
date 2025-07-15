// 负责“写”操作（增、改、删），即对数据库的数据进行变更。这里会用到 ActiveModel，通常是业务层的写接口。
use sea_orm::*;

pub struct MutationsService;

impl MutationsService {
	pub async fn create_env_config(
		db: &DbConn,
		env_config: entity::env_config::Model,
	) -> Result<String, DbErr> {
		let active_model = entity::env_config::ActiveModel {
			id: Set(env_config.id),
			name: Set(env_config.name),
			scope: Set(env_config.scope),
			description: Set(env_config.description),
			sort: Set(env_config.sort),
		};

		let res = active_model.insert(db).await?;
		Ok(res.id)
	}
}
