use log::info;
use sea_orm::DatabaseConnection;
use tauri::Manager;

use crate::db::setup_database;

pub mod command;
pub mod db;
pub mod entity;
pub mod environment_vars;
pub mod error;
pub mod model;
pub mod service;

pub struct AppState {
	pub db_conn: DatabaseConnection,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
	// db
	let db_conn = setup_database().await;

	tauri::Builder::default()
		.manage(AppState { db_conn })
		.plugin(tauri_plugin_notification::init())
		.plugin(tauri_plugin_os::init())
		.plugin(tauri_plugin_opener::init())
		.plugin(tauri_plugin_fs::init())
		.plugin(tauri_plugin_store::Builder::default().build())
		.plugin(tauri_plugin_clipboard_manager::init())
		.plugin(tauri_plugin_shell::init())
		.plugin(tauri_plugin_dialog::init())
		.plugin(tauri_plugin_updater::Builder::new().build())
		.plugin(
			tauri_plugin_log::Builder::default()
				.target(tauri_plugin_log::Target::new(
					tauri_plugin_log::TargetKind::Stdout,
				))
				.target(tauri_plugin_log::Target::new(
					tauri_plugin_log::TargetKind::LogDir {
						file_name: Some("logs".to_string()),
					},
				))
				.level_for("tauri", log::LevelFilter::Error)
				.level_for("hyper", log::LevelFilter::Off)
				.level_for("tracing", log::LevelFilter::Info)
				.level_for("sea_orm", log::LevelFilter::Info)
				.level_for("sqlx", log::LevelFilter::Off)
				.level_for("tao", log::LevelFilter::Off)
				.build(),
		)
		.setup(|app| {
			#[cfg(desktop)]
			let _ = app.handle()
				.plugin(tauri_plugin_single_instance::init(|app, args, cwd| {}));
			#[cfg(desktop)]
			let _ = app.handle()
				.plugin(tauri_plugin_window_state::Builder::default().build());
			#[cfg(desktop)]
			let _ = app.handle().plugin(tauri_plugin_autostart::init(
				tauri_plugin_autostart::MacosLauncher::LaunchAgent,
				Some(vec!["--flag1", "--flag2"]), /* 传递给应用程序的任意数量的参数 */
			));
			let main = app.get_webview_window("main").unwrap();
			let theme = main.theme().unwrap();
			info!("theme: {}", theme);
			Ok(())
		})
		.invoke_handler(tauri::generate_handler![
			// env_config
			command::env_config::list_env_configs,
			command::env_config::get_env_config,
			command::env_config::get_env_config_with_groups,
			command::env_config::create_env_config,
			command::env_config::create_env_config_transaction,
			command::env_config::update_env_config,
			command::env_config::delete_env_config,
			// variable_groups
			command::variable_group::list_variable_groups,
			command::variable_group::get_variable_group,
			command::variable_group::create_variable_group,
			command::variable_group::update_variable_group,
			command::variable_group::delete_variable_group,
			// environment_variables
			command::environment_variable::list_environment_variables,
			command::environment_variable::get_environment_variable,
			command::environment_variable::create_environment_variable,
			command::environment_variable::update_environment_variable,
			command::environment_variable::delete_environment_variable,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
