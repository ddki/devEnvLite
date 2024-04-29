interface Setting {
	language: string;
	homeDir: string;
	cacheDir: string;
	dataDir: string;
	envBackupDir: string;
}

interface ActiveConfig {
	activeConfigId: string;
	configNames: string[];
}

interface Config {
	id: string;
	scope: string;
	name: string;
	note?: string;
	sort: number;
	groupEnvs?: GroupEnv[];
}

interface GroupEnv {
	configId: string;
	id: string;
	name: string;
	note?: string;
	sort: number;
	envs?: Env[];
	envKeys?: string[];
	envAppliedCount?: number;
	envNotAppliedCount?: number;
}

interface Env {
	configId: string;
	groupId: string;
	key: string;
	value: string;
	sort: number;
	note?: string;
	isApplied?: boolean;
	isSame?: boolean;
	currentValue?: string;
}

export type { Setting, ActiveConfig, Config, GroupEnv, Env };
