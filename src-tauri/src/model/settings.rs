use lombok::{Getter, Setter};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_plugin_store::StoreExt;

#[derive(Debug, Clone, Serialize, Deserialize, Getter, Setter)]
pub struct Settings {
	language: String,
	#[serde(rename = "homeDir")]
	home_dir: String,
	#[serde(rename = "cacheDir")]
	cache_dir: String,
	#[serde(rename = "dataDir")]
	data_dir: String,
	#[serde(rename = "logDir")]
	log_dir: String,
	#[serde(rename = "envBackupDir")]
	env_backup_dir: String,
}

impl Settings {
	pub fn default<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Self {
		let home_dir = app
			.path()
			.app_local_data_dir()
			.expect("Failed to get home directory");
		let cache_dir = home_dir.join("cache");
		let data_dir = home_dir.join("data");
		let log_dir = app
			.path()
			.app_log_dir()
			.expect("Failed to get log directory");
		let env_backup_dir = home_dir.join("backup");
		let settings = Self {
			language: String::from("zh-CN"),
			home_dir: home_dir.into_os_string().to_string_lossy().to_string(),
			cache_dir: cache_dir.into_os_string().to_string_lossy().to_string(),
			data_dir: data_dir.into_os_string().to_string_lossy().to_string(),
			log_dir: log_dir.into_os_string().to_string_lossy().to_string(),
			env_backup_dir: env_backup_dir
				.into_os_string()
				.to_string_lossy()
				.to_string(),
		};
		settings.create_dir().expect("Failed to create directories");
		return settings;
	}

	pub fn load_from_store<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> anyhow::Result<Self> {
		let store = app.store("settings.json")?;
		let default_settings = Self::default(app);
		let language = store
			.get("language")
			.and_then(|v| v.as_str().map(String::from))
			.unwrap_or_else(|| default_settings.language);
		let home_dir = store
			.get("home_dir")
			.and_then(|v| v.as_str().map(String::from))
			.unwrap_or_else(|| default_settings.home_dir);
		let cache_dir = store
			.get("cache_dir")
			.and_then(|v| v.as_str().map(String::from))
			.unwrap_or_else(|| default_settings.cache_dir);
		let data_dir = store
			.get("data_dir")
			.and_then(|v| v.as_str().map(String::from))
			.unwrap_or_else(|| default_settings.data_dir);
		let log_dir = store
			.get("log_dir")
			.and_then(|v| v.as_str().map(String::from))
			.unwrap_or_else(|| default_settings.log_dir);
		let env_backup_dir = store
			.get("env_backup_dir")
			.and_then(|v| v.as_str().map(String::from))
			.unwrap_or_else(|| default_settings.env_backup_dir);
		Ok(Self {
			language,
			home_dir,
			cache_dir,
			data_dir,
			log_dir,
			env_backup_dir,
		})
	}

	fn create_dir(&self) -> anyhow::Result<()> {
		let home_dir = std::path::Path::new(&self.home_dir);
		let cache_dir = std::path::Path::new(&self.cache_dir);
		let data_dir = std::path::Path::new(&self.data_dir);
		let env_backup_dir = std::path::Path::new(&self.env_backup_dir);

		if !home_dir.exists() {
			std::fs::create_dir_all(home_dir)?;
		}
		if !cache_dir.exists() {
			std::fs::create_dir_all(cache_dir)?;
		}
		if !data_dir.exists() {
			std::fs::create_dir_all(data_dir)?;
		}
		if !env_backup_dir.exists() {
			std::fs::create_dir_all(env_backup_dir)?;
		}
		Ok(())
	}
}
