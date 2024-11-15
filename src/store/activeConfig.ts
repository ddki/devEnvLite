import { Store } from "@tauri-apps/plugin-store";
import type { ActiveConfig } from "./type";

const activeConfigStore = await Store.load("data/active-config.json");

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

const getActiveConfigNames = async (): Promise<string[]> => {
	return (await activeConfigStore.get("configNames")) as string[];
};

const getActiveConfigId = async (): Promise<string> => {
	return (await activeConfigStore.get("activeConfigId")) as string;
};

const setActiveConfigId = async (id: string): Promise<void> => {
	await activeConfigStore.set("activeConfigId", id);
	await activeConfigStore.save();
};

const pushActiveConfigName = async (name: string): Promise<void> => {
	const configNames = ((await activeConfigStore.get("configNames")) as string[]) || [];
	configNames.push(name);
	await activeConfigStore.set("configNames", configNames);
	await activeConfigStore.save();
};

const removeActiveConfigName = async (name: string): Promise<void> => {
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

export {
	getActiveConfig,
	saveActiveConfig,
	getActiveConfigNames,
	getActiveConfigId,
	setActiveConfigId,
	pushActiveConfigName,
	removeActiveConfigName,
	removeActiveId,
};
