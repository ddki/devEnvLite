// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log::{Target, TargetKind};

pub mod command;
pub mod environment_vars;

fn main() {
	tauri::Builder::default()
		.plugin(
			tauri_plugin_log::Builder::new()
				.targets([
					Target::new(TargetKind::Stdout),
					Target::new(TargetKind::LogDir {
						file_name: Some("log".to_string()),
					}),
					Target::new(TargetKind::Webview),
				])
				.build(),
		)
		.plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
			println!("{}, {argv:?}, {cwd}", app.package_info().name);
		}))
		.plugin(tauri_plugin_store::Builder::default().build())
		.plugin(tauri_plugin_clipboard_manager::init())
		.plugin(tauri_plugin_fs::init())
		.plugin(tauri_plugin_shell::init())
		.on_window_event(|_window, event| match event {
			_ => {}
		})
		.setup(|app| {
			// #[cfg(desktop)]
			// app.handle()
			// 	.plugin(tauri_plugin_updater::Builder::new().build())?;
			Ok(())
		})
		.invoke_handler(tauri::generate_handler![
			command::close_splashscreen,
			command::get_config_ids,
			command::get_keys,
			command::collate_envs,
			command::backup_envs,
			command::recover_envs,
			command::env_apply,
			command::group_env_apply,
			command::group_env_check,
			command::config_check,
			command::config_apply,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
