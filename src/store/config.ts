import { invoke } from "@tauri-apps/api/core";
import { v4 as uuidv4 } from "uuid";
import type { Config, Env, GroupEnv } from "./type";
import { pushActiveConfigName, removeActiveConfigName, removeActiveId } from "./index";

const getConfig = async (id: string): Promise<Config> => {
	const config = (await invoke("get_config", { configId: id })) as Config;
	console.log("getConfig: ", config);
	return config;
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
	config.groupEnvs = converterGroupEnvs(config.id, config.groupEnvs);
	console.log("saveConfig: ", config);
	await invoke("save_config", { configInfo: config })
		.then(async () => {
			await pushActiveConfigName(config.name);
		})
		.catch((e) => {
			console.log("saveConfig error: ", e);
			return false;
		});
	return true;
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
				if (newEnvs.length > 0) {
					item.envAppliedCount = newEnvs.filter((env) => env.isApplied).length;
					item.envNotAppliedCount = newEnvs.filter((env) => !env.isApplied).length;
				}
			} else {
				item.envs = [];
				item.envAppliedCount = 0;
				item.envNotAppliedCount = 0;
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
				if (groupEnv.envs && groupEnv.envs.length > 0) {
					if (!groupEnv.envAppliedCount) {
						item.envAppliedCount = groupEnv.envs.filter((env) => env.isApplied).length;
					}
					if (!groupEnv.envNotAppliedCount) {
						item.envAppliedCount = groupEnv.envs.filter((env) => !env.isApplied).length;
					}
				}
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
				item.isApplied = env.isApplied || false;
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
	const storeConfig = await getConfig(id);
	await invoke("remove_config", { configId: id })
		.then(async () => {
			// 移除名称
			await removeActiveConfigName(storeConfig.name);
			// 移除激活ID
			await removeActiveId(storeConfig.id);
		})
		.catch((e) => {
			console.log("deleteConfig error: ", e);
		});
};

const generateEnvs = (configId: string, groupId: string, envs: Map<string, string>): Env[] => {
	if (envs) {
		const envArray: Env[] = [];
		let sort = 0;
		for (const [key, value] of envs) {
			sort += 1;
			envArray.push({
				configId,
				groupId,
				key,
				value,
				sort,
				isApplied: true,
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
		name: "default",
		sort: 1,
	};
	groupEnv.envs = generateEnvs(generateConfig.id, groupEnv.id, envs);
	generateConfig.groupEnvs = [groupEnv];
	await saveConfig(generateConfig);
	return generateConfig;
};

export {
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
