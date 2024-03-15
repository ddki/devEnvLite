use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn close_splashscreen(app: AppHandle) {
	println!("close_splashscreen");
	// Close splashscreen
	if let Some(splashscreen) = app.get_webview_window("splashscreen") {
		splashscreen.close().unwrap();
	}
	// Show main window
	app.get_webview_window("main").unwrap().show().unwrap();
}
