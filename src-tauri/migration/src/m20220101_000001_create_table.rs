use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		// 环境变量表
		manager
			.create_table(
				Table::create()
					.table(EnvironmentVariable::Table)
					.if_not_exists()
					.col(pk_auto(EnvironmentVariable::Id))
					.col(
						ColumnDef::new(EnvironmentVariable::Key)
							.string()
							.not_null()
							.unique_key(),
					)
					.col(
						ColumnDef::new(EnvironmentVariable::Value)
							.string()
							.not_null(),
					)
					.col(ColumnDef::new(EnvironmentVariable::Description).string())
					.to_owned(),
			)
			.await?;

		// 环境变量配置表
		manager
			.create_table(
				Table::create()
					.table(EnvConfig::Table)
					.if_not_exists()
					.col(pk_auto(EnvConfig::Id))
					.col(
						ColumnDef::new(EnvConfig::Name)
							.string()
							.not_null()
							.unique_key(),
					)
					.col(
						ColumnDef::new(EnvConfig::Scope)
							.string()
							.not_null()
							.check("scope IN ('system', 'user')"),
					)
					.col(ColumnDef::new(EnvConfig::Description).string())
					.col(
						ColumnDef::new(EnvConfig::IsActive)
							.boolean()
							.default(true),
					)
					.col(ColumnDef::new(EnvConfig::Sort).integer().default(1))
					.to_owned(),
			)
			.await?;

		// 分组表
		manager
			.create_table(
				Table::create()
					.table(VariableGroup::Table)
					.if_not_exists()
					.col(pk_auto(VariableGroup::Id))
					.col(ColumnDef::new(VariableGroup::ConfigId).integer().not_null())
					.col(
						ColumnDef::new(VariableGroup::Name)
							.string()
							.not_null()
							.unique_key(),
					)
					.col(ColumnDef::new(VariableGroup::Description).string())
					.col(ColumnDef::new(VariableGroup::Sort).integer().default(1))
					.foreign_key(
						ForeignKey::create()
							.from_col(VariableGroup::ConfigId)
							.to(EnvConfig::Table, EnvConfig::Id),
					)
					.to_owned(),
			)
			.await?;

		// 环境变量-分组关联表
		manager
			.create_table(
				Table::create()
					.table(VariableGroupMapping::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(VariableGroupMapping::GroupId)
							.integer()
							.not_null(),
					)
					.col(
						ColumnDef::new(VariableGroupMapping::VariableId)
							.integer()
							.not_null(),
					)
					.col(
						ColumnDef::new(VariableGroupMapping::Sort)
							.integer()
							.default(1),
					)
					.primary_key(
						Index::create()
							.col(VariableGroupMapping::VariableId)
							.col(VariableGroupMapping::GroupId),
					)
					.foreign_key(
						ForeignKey::create()
							.from_col(VariableGroupMapping::VariableId)
							.to(EnvironmentVariable::Table, EnvironmentVariable::Id),
					)
					.foreign_key(
						ForeignKey::create()
							.from_col(VariableGroupMapping::GroupId)
							.to(VariableGroup::Table, VariableGroup::Id),
					)
					.to_owned(),
			)
			.await?;
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(VariableGroupMapping::Table).to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table(VariableGroup::Table).to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table(EnvConfig::Table).to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table(EnvironmentVariable::Table).to_owned())
			.await?;
		Ok(())
	}
}

#[derive(DeriveIden)]
enum EnvironmentVariable {
	#[sea_orm(iden = "environment_variable")]
	Table,
	#[sea_orm(iden = "id")]
	Id,
	#[sea_orm(iden = "key")]
	Key,
	#[sea_orm(iden = "value")]
	Value,
	#[sea_orm(iden = "description")]
	Description,
}

#[derive(DeriveIden)]
enum EnvConfig {
	#[sea_orm(iden = "env_config")]
	Table,
	#[sea_orm(iden = "id")]
	Id,
	#[sea_orm(iden = "name")]
	Name,
	#[sea_orm(iden = "scope")]
	Scope,
	#[sea_orm(iden = "description")]
	Description,
	#[sea_orm(iden = "is_active")]
	IsActive,
	#[sea_orm(iden = "sort")]
	Sort,
}

#[derive(DeriveIden)]
enum VariableGroup {
	#[sea_orm(iden = "variable_group")]
	Table,
	#[sea_orm(iden = "id")]
	Id,
	#[sea_orm(iden = "config_id")]
	ConfigId,
	#[sea_orm(iden = "name")]
	Name,
	#[sea_orm(iden = "description")]
	Description,
	#[sea_orm(iden = "sort")]
	Sort,
}

#[derive(DeriveIden)]
enum VariableGroupMapping {
	#[sea_orm(iden = "variable_group_mapping")]
	Table,
	#[sea_orm(iden = "group_id")]
	GroupId,
	#[sea_orm(iden = "variable_id")]
	VariableId,
	#[sea_orm(iden = "sort")]
	Sort,
}
