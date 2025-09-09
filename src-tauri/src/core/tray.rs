use log::info;
use tauri::{
	menu::{Menu, MenuItem},
	tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
	Manager,
};

pub fn init_tray(app: &tauri::App) -> anyhow::Result<()> {
	let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
	let menu = Menu::with_items(app, &[&quit_i])?;

	TrayIconBuilder::new()
		.tooltip(app.package_info().name.as_str())
		.icon(app.default_window_icon().unwrap().clone())
		.menu(&menu)
		.show_menu_on_left_click(false)
		.on_menu_event(|app, event| match event.id.as_ref() {
			"quit" => {
				info!("退出菜单项目被点击");
				app.exit(0);
			}
			"show_main" => {
				info!("显示主窗口菜单项目被点击");
				let app = app.app_handle();
				if let Some(window) = app.get_webview_window("main") {
					let _ = window.unminimize();
					let _ = window.show();
					let _ = window.set_focus();
				}
			}
			_ => {
				info!("菜单项目 {:?} 未处理", event.id);
			}
		})
		.on_tray_icon_event(|tray, event| match event {
			TrayIconEvent::Click {
				button: MouseButton::Left,
				button_state: MouseButtonState::Up,
				..
			} => {
				info!("托盘图标被点击");
				// 当点击托盘图标时，将展示并聚焦于主窗口
				let app = tray.app_handle();
				if let Some(window) = app.get_webview_window("main") {
					let _ = window.unminimize();
					let _ = window.show();
					let _ = window.set_focus();
				}
			}
			_ => {
				info!("未处理的托盘图标事件 {event:?}");
			}
		})
		.build(app)?;
	Ok(())
}
