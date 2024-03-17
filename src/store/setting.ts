import { Store } from "@tauri-apps/plugin-store";

const store = new Store("config/settings.json");

const getSetting = async (): Promise<Setting> => {
	return {
		language: await store.get("language"),
		theme: await store.get("theme"),
		homeDir: await store.get("homeDir"),
		cacheDir: await store.get("cacheDir"),
		dataDir: await store.get("dataDir"),
		envBackupDir: await store.get("envBackupDir"),
	};
};

const saveSetting = async (setting: Setting): Promise<boolean> => {
	try {
		await store.set("language", setting.language);
		await store.set("theme", setting.theme);
		await store.set("homeDir", setting.homeDir);
		await store.set("cacheDir", setting.cacheDir);
		await store.set("dataDir", setting.dataDir);
		await store.set("envBackupDir", setting.envBackupDir);

		await store.save();
		return true;
	} catch (e) {
		return false;
	}
};

export { saveSetting, getSetting };
