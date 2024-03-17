import { Store } from "@tauri-apps/plugin-store";

const activeConfigStore = new Store("active-config.json");

const getActiveConfig = async (): Promise<ActiveConfig> => {
	return {
		activeConfigId: (await activeConfigStore.get("activeConfigId")) as string,
		configNames: (await activeConfigStore.get("configNames")) as string[],
	};
};

const saveActiveConfig = async (config: ActiveConfig): Promise<boolean> => {
	await activeConfigStore.set("activeConfigId", config.activeConfigId || "");
	await activeConfigStore.set("configNames", config.configNames || []);
	await activeConfigStore.save();
	return true;
};

const getConfigNames = async (): Promise<string[]> => {
	return (await activeConfigStore.get("configNames")) as string[];
};

const getActiveConfigId = async (): Promise<string> => {
	return (await activeConfigStore.get("activeConfigId")) as string;
};

const setActiveConfigId = async (id: string): Promise<void> => {
	await activeConfigStore.set("activeConfigId", id);
};

const pushConfigName = async (name: string): Promise<void> => {
	const configNames = ((await activeConfigStore.get("configNames")) as string[]) || [];
	configNames.push(name);
	await activeConfigStore.set("configNames", configNames);
};

const getConfig = async (id: string): Promise<Config> => {
	const path = `config/${id}.json`;
	const store = new Store(path);
	const storeId = (await store.get("id")) as string;
	const storeName = (await store.get("name")) as string;

	return {
		id: storeId,
		name: storeName,
		note: (await store.get("note")) as string,
		sort: (await store.get("sort")) as number,
		groupEnvs: (await store.get("groupEnvs")) || [],
	};
};

const getConfigs = async (ids: string[]): Promise<Config[]> => {
	const configs: Config[] = [];
	for (const id of ids) {
		const config = await getConfig(id);
		configs.push(config);
	}
	return configs;
};

const saveConfig = async (config: Config): Promise<boolean> => {
	const path = `config/${config.id}.json`;
	const store = new Store(path);
	const load = await loadConfig(store, config);
	if (load) {
		await store.set("id", config.id);
		await store.set("name", config.name);
		await store.set("note", config.note);
		await store.set("sort", config.sort);
		await store.set("groupEnvs", config.groupEnvs || []);
		await store.save();
	}
	return load;
};

const loadConfig = async (store: Store, config: Config): Promise<boolean> => {
	await store.set("id", config.id);
	await store.save();
	const load = await store.has("id");
	if (!load) {
		loadConfig(store, config);
	}
	return load;
};

export {
	getActiveConfig,
	saveActiveConfig,
	getConfigNames,
	getActiveConfigId,
	setActiveConfigId,
	pushConfigName,
	getConfig,
	getConfigs,
	saveConfig,
};
