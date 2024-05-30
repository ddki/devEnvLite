use log::debug;
use lombok::{Getter, Setter};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_plugin_store::StoreBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, Getter, Setter)]
pub struct Settings {
	language: String,
	#[serde(rename = "homeDir")]
	home_dir: String,
	#[serde(rename = "cacheDir")]
	cache_dir: String,
	#[serde(rename = "dataDir")]
	data_dir: String,
	#[serde(rename = "envBackupDir")]
	env_backup_dir: String,
}

impl Settings {
	pub fn default<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Self {
		let home_dir = app
			.path()
			.app_data_dir()
			.expect("Failed to get home directory");
		let cache_dir = home_dir.join("cache");
		let data_dir = home_dir.join("data");
		let env_backup_dir = home_dir.join("backup");
		Self {
			language: String::from("en"),
			home_dir: home_dir.into_os_string().to_string_lossy().to_string(),
			cache_dir: cache_dir.into_os_string().to_string_lossy().to_string(),
			data_dir: data_dir.into_os_string().to_string_lossy().to_string(),
			env_backup_dir: env_backup_dir
				.into_os_string()
				.to_string_lossy()
				.to_string(),
		}
	}

	pub fn build_form_store<R: tauri::Runtime>(
		app: tauri::AppHandle<R>,
		store: &tauri_plugin_store::Store<R>,
	) -> Self {
		let store_language = store
			.get("language")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_else(|| "en".to_string());

		let home_dir = app
			.path()
			.app_data_dir()
			.expect("Failed to get home directory");

		let store_home_dir = store
			.get("homeDir")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_else(|| {
				home_dir
					.clone()
					.into_os_string()
					.to_string_lossy()
					.to_string()
			});

		let store_cache_dir = store
			.get("cacheDir")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_else(|| {
				home_dir
					.clone()
					.join("cache")
					.into_os_string()
					.to_string_lossy()
					.to_string()
			});

		let store_data_dir = store
			.get("dataDir")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_else(|| {
				home_dir
					.clone()
					.join("data")
					.into_os_string()
					.to_string_lossy()
					.to_string()
			});

		let store_env_backup_dir = store
			.get("envBackupDir")
			.and_then(|v| v.as_str())
			.map(|s| s.to_string())
			.unwrap_or_else(|| {
				home_dir
					.clone()
					.join("backup")
					.into_os_string()
					.to_string_lossy()
					.to_string()
			});

		Self {
			language: store_language,
			home_dir: store_home_dir,
			cache_dir: store_cache_dir,
			data_dir: store_data_dir,
			env_backup_dir: store_env_backup_dir,
		}
	}

	pub fn save_to_store<R: tauri::Runtime>(&self, mut store: tauri_plugin_store::Store<R>,) -> anyhow::Result<()> {
		store.insert("language".to_string(), self.language.as_str().into())?;
		store.insert("homeDir".to_string(), self.home_dir.as_str().into())?;
		store.insert("cacheDir".to_string(), self.cache_dir.as_str().into())?;
		store.insert("dataDir".to_string(), self.data_dir.as_str().into())?;
		store.insert("envBackupDir".to_string(), self.env_backup_dir.as_str().into())?;
		store.save().expect("Failed to save settings to store");
		Ok(())
	}

	pub fn load_from_store<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> anyhow::Result<Self> {
		let mut store = StoreBuilder::new("settings.json").build(app.clone());
		match store.load() {
				Ok(()) => {
					let settings = Self::build_form_store(app, &store);
					settings.save_to_store(store)?;
					settings.create_dir()?;
					Ok(settings)
				}
				Err(e) => {
					debug!("Failed to load settings from store: {}", e);
					let settings = Self::default(app);
					settings.save_to_store(store)?;
					Ok(settings)
				}
		}
	}

	pub fn create_dir(&self) -> anyhow::Result<()> {
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
