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
}

interface Env {
	groupId: string;
	key: string;
	value: string;
	sort: number;
	note?: string;
}
