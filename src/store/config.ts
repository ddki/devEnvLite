import { Store } from "@tauri-apps/plugin-store";

const activeConfigStore = new Store("active-config.json");

const getActiveConfig = async (): Promise<ActiveConfig> => {
	return {
		activeConfigId: (await activeConfigStore.get("language")) || "",
	};
};

const saveActiveConfig = async (config: ActiveConfig): Promise<boolean> => {
	try {
		await activeConfigStore.set("activeConfigId", config.activeConfigId);

		await activeConfigStore.save();
		return true;
	} catch (e) {
		return false;
	}
};

const getConfig = async (id: string): Promise<Config> => {
	const store = new Store(`configs/${id}.json`);
	return {
		id: (await store.get("id")) || "",
		name: (await store.get("name")) || "",
		note: (await store.get("note")) || "",
		sort: (await store.get("sort")) || 0,
		groupEnvs: (await store.get("groupEnvs")) || [],
	};
};

const saveConfig = async (config: Config): Promise<boolean> => {
	const store = new Store(`configs/${config.id}.json`);
	try {
		await store.set("id", config.id);
		await store.set("name", config.name);
		await store.set("note", config.note);
		await store.set("sort", config.sort);
		await store.set("groupEnvs", config.groupEnvs);

		await store.save();
		return true;
	} catch (e) {
		return false;
	}
};

export { getActiveConfig, saveActiveConfig, getConfig, saveConfig };
