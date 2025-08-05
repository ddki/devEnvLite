use log::{error, info};
use sea_orm::DatabaseConnection;
use tauri::Manager;

use crate::db::setup_database;

pub mod command;
pub mod core;
pub mod db;
pub mod entity;
pub mod environment_vars;
pub mod error;
pub mod model;
pub mod service;

pub struct AppState {
	pub db_conn: DatabaseConnection,
}

#[cfg(desktop)]
#[tokio::main]
pub async fn run() {
	// db
	let db_conn = setup_database().await;
	app_init::setup_plugins(tauri::Builder::default())
		.manage(AppState { db_conn })
		.on_window_event(|window, event| {
			if let tauri::WindowEvent::CloseRequested { api, .. } = event {
				info!("窗口关闭...");
				window.hide().unwrap();
				api.prevent_close();
			}
		})
		.setup(|app| {
			info!("应用初始化...");
			if let Err(e) = app_init::setup_autostart(app) {
				error!("设置自启动失败: {:?}", e);
			}
			if let Err(e) = app_init::setup_window_state(app) {
				error!("设置窗口状态失败: {:?}", e);
			}
			if let Err(e) = app_init::setup_single_instance(app) {
				error!("设置单实例失败: {:?}", e);
			}
			if let Err(e) = core::tray::init_tray(app) {
				error!("初始化托盘失败: {:?}", e);
			}
			app_init::init_core_async(app.handle());
			let main = app.get_webview_window("main").unwrap();
			let theme = main.theme().unwrap();
			info!("主题: {}", theme);

			info!("应用初始化完成");
			Ok(())
		})
		.invoke_handler(app_init::generate_handlers())
		.run(tauri::generate_context!())
		.expect("应用启动失败");
}

mod app_init {
	use anyhow::Ok;

	use super::*;
	pub fn setup_plugins(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
		let mut builder = builder
			.plugin(tauri_plugin_notification::init())
			.plugin(tauri_plugin_os::init())
			.plugin(tauri_plugin_opener::init())
			.plugin(tauri_plugin_fs::init())
			.plugin(tauri_plugin_store::Builder::default().build())
			.plugin(tauri_plugin_clipboard_manager::init())
			.plugin(tauri_plugin_shell::init())
			.plugin(tauri_plugin_dialog::init())
			.plugin(tauri_plugin_updater::Builder::new().build());
		// 日志插件与devtools插件冲突，二选一
		#[cfg(not(debug_assertions))]
		{
			builder = builder.plugin(
				tauri_plugin_log::Builder::default()
					.targets([
						tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
						tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
							file_name: None,
						}),
						tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
					])
					.level_for("tauri", log::LevelFilter::Error)
					.level_for("hyper", log::LevelFilter::Off)
					.level_for("tracing", log::LevelFilter::Info)
					.level_for("sea_orm", log::LevelFilter::Info)
					.level_for("sqlx", log::LevelFilter::Off)
					.level_for("tao", log::LevelFilter::Off)
					.build(),
			);
		}
		#[cfg(debug_assertions)]
		{
			builder = builder.plugin(tauri_plugin_devtools::init());
		}
		builder
	}

	/// 初始化单实例插件
	pub fn setup_single_instance(app: &tauri::App) -> anyhow::Result<()> {
		#[cfg(desktop)]
		app.handle()
			.plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
				use tauri_plugin_notification::NotificationExt;

				println!("{}, {args:?}, {cwd}", app.package_info().name);
				app.get_webview_window("main").unwrap().show().unwrap();
				app.notification()
					.builder()
					.title("应用程序已经在运行了!")
					.body("你可以在托盘图标中找到它。")
					.show()
					.unwrap();
			}))?;
		Ok(())
	}

	/// 初始化自启动插件
	pub fn setup_autostart(app: &tauri::App) -> anyhow::Result<()> {
		#[cfg(target_os = "macos")]
		let mut auto_start_plugin_builder = tauri_plugin_autostart::Builder::new();
		#[cfg(not(target_os = "macos"))]
		let auto_start_plugin_builder = tauri_plugin_autostart::Builder::new();

		#[cfg(target_os = "macos")]
		{
			auto_start_plugin_builder = auto_start_plugin_builder
				.macos_launcher(MacosLauncher::LaunchAgent)
				.app_name(app.config().identifier.clone());
		}
		app.handle().plugin(auto_start_plugin_builder.build())?;
		Ok(())
	}

	/// 初始化窗口状态管理插件
	pub fn setup_window_state(app: &tauri::App) -> anyhow::Result<()> {
		info!("初始化窗口状态管理...");
		let window_state_plugin = tauri_plugin_window_state::Builder::new()
			.with_filename("window_state.json")
			.with_state_flags(tauri_plugin_window_state::StateFlags::default())
			.build();
		app.handle().plugin(window_state_plugin)?;
		Ok(())
	}

	/// 生成命令处理函数
	pub fn generate_handlers(
	) -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + 'static {
		tauri::generate_handler![
			// os_environment_variable
			command::os_environment_variable::get_os_environment_variables,
			command::os_environment_variable::get_os_environment_variable_keys,
			command::os_environment_variable::collate_os_environment_variables,
			command::os_environment_variable::backup_os_environment_variables,
			command::os_environment_variable::recover_os_environment_variables,
			// settings
			command::settings::get_settings,
			command::settings::save_settings,
			// env_config
			command::env_config::list_env_configs,
			command::env_config::list_active_env_configs,
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
		]
	}

	/// 异步初始化核心组件
	pub fn init_core_async(app_handle: &tauri::AppHandle) {
		let async_app_handle = app_handle.clone();
		tokio::spawn(async move {
			// todo 在这里调用 command::settings::get_settings(app_handle.clone()) 并打印结果
			let settings = command::settings::get_settings(async_app_handle).await;
			info!("settings: {:?}", settings);
		});
	}
}
