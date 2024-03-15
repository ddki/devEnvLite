import { Store } from "@tauri-apps/plugin-store";

interface Setting {
	language: string | null;
	dataDir: string | null;
	envBackupDir: string | null;
	theme: string | null;
}
const store = new Store(".settings");

const getSetting = async (): Promise<Setting> => {
	return {
		language: await store.get("language"),
		dataDir: await store.get("dataDir"),
		envBackupDir: await store.get("envBackupDir"),
		theme: await store.get("theme"),
	};
};

const saveSetting = async (setting: Setting): Promise<boolean> => {
	try {
		await store.set("language", setting.language);
		await store.set("dataDir", setting.dataDir);
		await store.set("envBackupDir", setting.envBackupDir);
		await store.set("theme", setting.theme);
		await store.save();
		return true;
	} catch (e) {
		return false;
	}
};

export { saveSetting, getSetting };
