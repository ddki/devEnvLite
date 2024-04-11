import { Store } from "@tauri-apps/plugin-store";
import type { Setting } from "./type";

const store = new Store("settings.json");

const getSetting = async (): Promise<Setting> => {
	return {
		language: (await store.get("language")) as string,
		homeDir: (await store.get("homeDir")) as string,
		cacheDir: (await store.get("cacheDir")) as string,
		dataDir: (await store.get("dataDir")) as string,
		envBackupDir: (await store.get("envBackupDir")) as string,
	};
};

const saveSetting = async (setting: Setting): Promise<boolean> => {
	try {
		await store.set("language", setting.language);
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
