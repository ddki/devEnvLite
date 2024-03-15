// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log::{Target, TargetKind};

pub mod command;

fn main() {
	tauri::Builder::default()
		.plugin(
			tauri_plugin_log::Builder::new()
				.targets([
					Target::new(TargetKind::Stdout),
					Target::new(TargetKind::LogDir {
						file_name: Some("DevEnvLite.log".to_string()),
					}),
					Target::new(TargetKind::Webview),
				])
				.build(),
		)
		.plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
			println!("{}, {argv:?}, {cwd}", app.package_info().name);
		}))
		.plugin(tauri_plugin_window_state::Builder::default().build())
		.plugin(tauri_plugin_store::Builder::default().build())
		.plugin(tauri_plugin_fs::init())
		.plugin(tauri_plugin_shell::init())
		.setup(|app| {
			// #[cfg(desktop)]
			// app.handle()
			// 	.plugin(tauri_plugin_updater::Builder::new().build())?;
			// 将所有打开窗口的状态保存到磁盘
			// app.save_window_state(StateFlags::all());
			Ok(())
		})
		.invoke_handler(tauri::generate_handler![command::close_splashscreen])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
