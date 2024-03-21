import { BaseDirectory, remove } from "@tauri-apps/plugin-fs";
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
	await activeConfigStore.save();
};

const pushConfigName = async (name: string): Promise<void> => {
	const configNames = ((await activeConfigStore.get("configNames")) as string[]) || [];
	configNames.push(name);
	await activeConfigStore.set("configNames", configNames);
	await activeConfigStore.save();
};

const removeConfigName = async (name: string): Promise<void> => {
	const configNames = ((await activeConfigStore.get("configNames")) as string[]) || [];
	const newConfigNames = configNames.filter((item) => item !== name);
	await activeConfigStore.set("configNames", newConfigNames);
	await activeConfigStore.save();
};

const removeActiveId = async (configId: string): Promise<void> => {
	const activeConfigId = (await activeConfigStore.get("activeConfigId")) as string;
	if (configId === activeConfigId) {
		await setActiveConfigId("");
	}
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

const getGroupEnvs = async (configId: string): Promise<GroupEnv[]> => {
	const config = await getConfig(configId);
	return config.groupEnvs || [];
};

const getGroupEnv = async (configId: string, groupEnvId: string): Promise<GroupEnv | null> => {
	const groupEnvs = await getGroupEnvs(configId);
	for (const item of groupEnvs) {
		if (item.id === groupEnvId) {
			return item;
		}
	}
	return null;
};

const getEnvs = async (configId: string, groupEnvId: string): Promise<Env[]> => {
	const groupEnv = await getGroupEnv(configId, groupEnvId);
	return groupEnv?.envs || [];
};

const getEnv = async (
	configId: string,
	groupEnvId: string,
	envKey: string,
): Promise<Env | null> => {
	const envs = await getEnvs(configId, groupEnvId);
	for (const env of envs) {
		if (env.key === envKey) {
			return env;
		}
	}
	return null;
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
		await store.set("groupEnvs", converterGroupEnvs(config.id, config.groupEnvs));
		await store.save();

		await pushConfigName(config.name);
	}
	return load;
};

const converterGroupEnvs = (configId: string, groupEnvs: GroupEnv[] | undefined): GroupEnv[] => {
	if (groupEnvs) {
		return groupEnvs.map((item) => {
			item.configId = configId;
			if (item.envs) {
				const newEnvs = item.envs.map((env) => {
					env.groupId = item.id;
					return env;
				});
				item.envs = newEnvs;
			} else {
				item.envs = [];
			}
			return item;
		});
	}
	return [];
};

const saveEnvToGroup = async (configId: string, groupEnvId: string, env: Env): Promise<boolean> => {
	const config = await getConfig(configId);
	const groups = await getGroupEnvs(configId);
	const group = await getGroupEnv(configId, groupEnvId);
	if (group) {
		const envs = group?.envs || [];
		envs.push(env);
		group.envs = envs;
	} else {
		return false;
	}
	const newGroups = groups.map((item) => {
		if (item.id === group?.id) {
			return group;
		}
		return item;
	});
	config.groupEnvs = newGroups;
	return await saveConfig(config);
};

const deleteEnv = async (
	configId: string,
	groupEnvId: string,
	envKey: string,
): Promise<boolean> => {
	const config = await getConfig(configId);
	const groups = await getGroupEnvs(configId);
	const group = await getGroupEnv(configId, groupEnvId);
	if (group) {
		const newEnvs = (group.envs || []).filter((item) => item.key !== envKey);
		group.envs = newEnvs;
	} else {
		return false;
	}
	const newGroups = groups.map((item) => {
		if (item.id === group?.id) {
			return group;
		}
		return item;
	});
	config.groupEnvs = newGroups;
	return await saveConfig(config);
};

const deleteConfig = async (config: Config): Promise<void> => {
	const path = `config/${config.id}.json`;
	await remove(path, { baseDir: BaseDirectory.AppData });
	// 移除名称
	await removeConfigName(config.name);
	// 移除激活ID
	await removeActiveId(config.id);
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
	getGroupEnvs,
	getGroupEnv,
	getEnvs,
	getEnv,
	saveConfig,
	saveEnvToGroup,
	deleteConfig,
	deleteEnv,
};
