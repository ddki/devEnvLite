// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

pub mod command;
pub mod entity;
pub mod environment_vars;
pub mod error;
pub mod model;
pub mod service;

fn main() {
	let mut ctx = tauri::generate_context!();
	tauri::Builder::default()
		.plugin(tauri_plugin_notification::init())
		.plugin(
			tauri_plugin_log::Builder::new()
				.targets([
					Target::new(TargetKind::Stdout),
					Target::new(TargetKind::LogDir {
						file_name: Some("devEnvLite".to_string()),
					}),
					Target::new(TargetKind::Webview),
				])
				// how to set log level: https://github.com/tauri-apps/plugins-workspace/issues/36
				// .level(LevelFilter::Debug)
				.build(),
		)
		.plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
			info!("{}, {argv:?}, {cwd}", app.package_info().name);
		}))
		.plugin(tauri_plugin_os::init())
		.plugin(tauri_plugin_opener::init())
		.plugin(tauri_plugin_fs::init())
		.plugin(tauri_plugin_store::Builder::default().build())
		.plugin(tauri_plugin_clipboard_manager::init())
		.plugin(tauri_plugin_fs::init())
		.plugin(tauri_plugin_shell::init())
		.plugin(tauri_plugin_dialog::init())
		.plugin(tauri_plugin_updater::Builder::new().build())
		.on_window_event(|_window, event| match event {
			_ => {}
		})
		.setup(|app| {
			// #[cfg(desktop)]
			// let _ = app.handle().plugin(tauri_plugin_single_instance::init(|app, args, cwd| {}));
			#[cfg(desktop)]
			let _ = app.handle()
				.plugin(tauri_plugin_window_state::Builder::default().build());
			#[cfg(desktop)]
			let _ = app.handle().plugin(tauri_plugin_autostart::init(
				tauri_plugin_autostart::MacosLauncher::LaunchAgent,
				Some(vec!["--flag1", "--flag2"]), /* 传递给应用程序的任意数量的参数 */
			));

			// #[cfg(desktop)]
			// app.handle()
			// 	.plugin(tauri_plugin_updater::Builder::new().build())?;
			let main = app.get_webview_window("main").unwrap();
			let theme = main.theme().unwrap();
			info!("theme: {}", theme);
			Ok(())
		})
		.invoke_handler(tauri::generate_handler![
			command::get_config_ids,
			command::get_envs,
			command::get_keys,
			command::collate_envs,
			command::backup_envs,
			command::recover_envs,
			command::env_apply,
			command::group_env_apply,
			command::config_check,
			command::config_apply,
			command::get_config,
			command::save_config,
			command::remove_config,
			command::config_export,
			command::import_config_form_file,
			command::import_config_form_url,
		])
		.run(ctx)
		.expect("error while running tauri application");
}
