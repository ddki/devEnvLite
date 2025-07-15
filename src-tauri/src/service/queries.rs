use sea_orm::*;

// 负责“读”操作（查），即从数据库获取数据。这里会用到 Entity，通常是业务层的查询接口
pub struct QueriesService;

impl QueriesService {
	pub async fn list_env_configs(db: &DbConn) -> Result<Vec<entity::env_config::Model>, DbErr> {
		entity::env_config::Entity::find().all(db).await
	}
}
