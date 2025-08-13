use crate::service::sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use tauri::Runtime;

pub async fn setup_database<R: Runtime>(
	#[cfg(not(debug_assertions))] app_handle: &tauri::AppHandle<R>,
) -> DatabaseConnection {
	let db_url = get_database_url::<R>(
		#[cfg(not(debug_assertions))]
		app_handle,
	);
	let db_conn = Database::connect(&db_url)
		.await
		.expect(&format!("Error connecting to {}", &db_url));

	Migrator::up(&db_conn, None)
		.await
		.expect("unable to run migrations");

	return db_conn;
}

fn get_database_url<R: Runtime>(
	#[cfg(not(debug_assertions))] app_handle: &tauri::AppHandle<R>,
) -> String {
	#[cfg(debug_assertions)]
	{
		dotenvy::dotenv().ok();
		std::env::var("DATABASE_URL").unwrap()
	}

	#[cfg(not(debug_assertions))]
	{
		use crate::model::Settings;
		use std::path::PathBuf;

		let settings = Settings::load_from_store(app_handle).unwrap();
		let data_dir = settings.get_data_dir();
		let data_path = PathBuf::from(data_dir);
		let db_path = data_path.join("db.sqlite");
		format!("sqlite://{}?mode=rwc", db_path.to_string_lossy())
	}
}
