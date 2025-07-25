use sea_orm::Database;
// Add the following imports or definitions for VariableGroup and EnvironmentVariable
use app_lib::model::{EnvConfig, EnvironmentVariable, VariableGroup};
use app_lib::service::TransactionService;

#[tokio::test]
async fn test_create_env_config() {
	// 这里可以添加测试代码，模拟数据库连接和事务操作
	// 使用 Mock 或者实际的测试数据库来验证 create_env_config 的行为
	dotenvy::dotenv().ok();
	let database_url = std::env::var("DATABASE_URL").unwrap();
	let db = Database::connect(database_url).await.unwrap();

	let config_id = ulid::Ulid::new().to_string();
	let mut groups = Vec::<VariableGroup>::new();
	for i in 1..=5 {
		let mut variables = Vec::<EnvironmentVariable>::new();
		for j in 1..=3 {
			variables.push(EnvironmentVariable {
				id: ulid::Ulid::new().to_string(),
				key: format!("VAR_{}_{}", i, j),
				value: format!("Value for group {} var {}", i, j),
				description: Some(format!("Description for group {} var {}", i, j)),
				sort: Some(j as i32),
			});
		}
		groups.push(VariableGroup {
			id: ulid::Ulid::new().to_string(),
			config_id: config_id.clone(),
			name: format!("Group {}", i),
			description: Some(format!("Description for group {}", i)),
			sort: Some(i as i32),
			variables: None, // 可以添加测试用的变量
		});
	}
	let config = EnvConfig {
		id: config_id.clone(),
		name: "Test Config".to_string(),
		scope: "global".to_string(),
		description: Some("This is a test config".to_string()),
		is_active: true,
		sort: Some(1),
		groups: Some(groups), // 可以添加测试用的变量组
	};
	let result = TransactionService::create_env_config(&db, config).await;
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), "test_config");
}
