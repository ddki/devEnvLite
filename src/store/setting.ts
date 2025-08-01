import { path } from "@tauri-apps/api";
import { BaseDirectory, mkdir } from "@tauri-apps/plugin-fs";
import { Store } from "@tauri-apps/plugin-store";
import type { Setting } from "./type";

const store = await Store.load("settings.json");

const getSetting = async (): Promise<Setting> => {
	return {
		language: (await store.get<string>("language")) || "zh-CN",
		homeDir: (await store.get<string>("homeDir")) || (await path.appDataDir()),
		cacheDir: (await store.get<string>("cacheDir")) || (await path.appCacheDir()),
		dataDir: (await store.get<string>("dataDir")) || (await path.appDataDir()),
		logDir: (await store.get<string>("logDir")) || (await path.appLogDir()),
		envBackupDir: (await store.get<string>("envBackupDir")) || "",
	};
};

const createDir = async (dirName: string): Promise<string> => {
	console.log("createDir", dirName);
	await mkdir(dirName, { baseDir: BaseDirectory.AppData });
	const dirPath = path.join(await path.appDataDir(), dirName);
	console.log("createDir", dirPath);
	return dirPath;
};

const saveSetting = async (setting: Setting): Promise<boolean> => {
	try {
		await store.set("language", setting.language);
		await store.set("homeDir", setting.homeDir);
		await store.set("cacheDir", setting.cacheDir);
		await store.set("dataDir", setting.dataDir);
		await store.set("logDir", setting.logDir);
		await store.set("envBackupDir", setting.envBackupDir);

		await store.save();
		return true;
	} catch (e) {
		return false;
	}
};

export { saveSetting, getSetting };
