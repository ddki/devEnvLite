import { BaseDirectory, remove } from "@tauri-apps/plugin-fs";
import { Store } from "@tauri-apps/plugin-store";
import { v4 as uuidv4 } from "uuid";
import type { ActiveConfig, Config, Env, GroupEnv } from "./type";

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

const getGroupEnv = async (configId: string, groupEnvId: string): Promise<GroupEnv> => {
	const groupEnvs = await getGroupEnvs(configId);
	if (groupEnvs.length > 0) {
		for (const item of groupEnvs) {
			if (item.id === groupEnvId) {
				return item;
			}
		}
	}
	return {} as GroupEnv;
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
		const storeConfig = await getConfig(config.id);
		if (storeConfig) {
			await store.set("id", storeConfig.id);
			await store.set("name", config.name);
			await store.set("note", config.note);
			await store.set("sort", config.sort);
			const groupEnvs = converterGroupEnvs(
				storeConfig.id,
				config.groupEnvs || storeConfig.groupEnvs,
			);
			console.log("[saveConfig()] groupEnvs: ", groupEnvs);
			await store.set("groupEnvs", groupEnvs);
		} else {
			await store.set("id", config.id);
			await store.set("name", config.name);
			await store.set("note", config.note);
			await store.set("sort", config.sort);
			await store.set("groupEnvs", converterGroupEnvs(config.id, config.groupEnvs));
		}
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

const saveGroupEnvToConfig = async (groupEnv: GroupEnv): Promise<boolean> => {
	const config = await getConfig(groupEnv.configId);
	const groups = await getGroupEnvs(groupEnv.configId);
	const group = await getGroupEnv(groupEnv.configId, groupEnv.id);
	console.log("saveGroupEnvToConfig: groups, group:", groups, group);
	if (group?.id) {
		// 存在，更新
		const newGroups = groups.map((item) => {
			if (item.id === groupEnv.id) {
				item.name = groupEnv.name;
				item.note = groupEnv.note;
				item.sort = groupEnv.sort;
				item.envs = groupEnv.envs;
			}
			return item;
		});
		console.log("saveGroupEnvToConfig: ", newGroups);
		config.groupEnvs = newGroups;
	} else {
		// 不存在，添加
		groups.push(groupEnv);
		config.groupEnvs = groups;
	}
	return await saveConfig(config);
};

const saveEnvToGroup = async (configId: string, env: Env): Promise<boolean> => {
	console.log("saveEnvToGroup param: ", configId, env);
	const storeEnvs = await getEnvs(configId, env.groupId);
	const storeEnv = await getEnv(configId, env.groupId, env.key);
	const group = await getGroupEnv(configId, env.groupId);
	console.log("saveEnvToGroup envs, storeEnv, group: ", storeEnvs, storeEnv, group);
	if (storeEnv?.key) {
		// 存在，更新
		const newEnvs = storeEnvs.map((item) => {
			if (item.key === env.key) {
				item.value = env.value;
				item.note = env.note;
				item.sort = env.sort;
			}
			return item;
		});
		group.envs = newEnvs;
		console.log("saveEnvToGroup newEnvs:", newEnvs);
	} else {
		// 不存在，添加
		storeEnvs.push(env);
		group.envs = storeEnvs;
		console.log("saveEnvToGroup envs:", storeEnvs);
	}
	console.log("saveEnvToGroup group:", group);
	return await saveGroupEnvToConfig(group);
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

const deleteGroupEnv = async (configId: string, groupId: string): Promise<boolean> => {
	const config = await getConfig(configId);
	const groups = await getGroupEnvs(configId);
	const newGroups = groups.filter((item) => item.id !== groupId);
	config.groupEnvs = newGroups;
	return await saveConfig(config);
};

const deleteConfig = async (id: string): Promise<void> => {
	const path = `config/${id}.json`;
	await remove(path, { baseDir: BaseDirectory.AppData });
	const storeConfig = await getConfig(id);
	// 移除名称
	await removeConfigName(storeConfig.name);
	// 移除激活ID
	await removeActiveId(storeConfig.id);
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

const generateEnvs = (groupId: string, envs: Map<string, string>): Env[] => {
	if (envs) {
		const envArray: Env[] = [];
		let sort = 0;
		for (const [key, value] of envs) {
			sort += 1;
			envArray.push({
				groupId,
				key,
				value,
				sort,
			});
		}
		return envArray;
	}
	return [];
};

const generateConfigFromEnvs = async (
	config: Config,
	envs: Map<string, string>,
): Promise<Config> => {
	console.log("generateConfigFromEnvs config, envs", config, envs);
	const generateConfig = {
		...config,
	};
	const groupEnv: GroupEnv = {
		configId: generateConfig.id,
		id: uuidv4(),
		name: "auto generate",
		sort: 1,
	};
	groupEnv.envs = generateEnvs(groupEnv.id, envs);
	generateConfig.groupEnvs = [groupEnv];
	await saveConfig(generateConfig);
	return generateConfig;
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
	saveGroupEnvToConfig,
	saveEnvToGroup,
	deleteConfig,
	deleteGroupEnv,
	deleteEnv,
	generateConfigFromEnvs,
};
