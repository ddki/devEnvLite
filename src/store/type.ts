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
	id: string;
	name: string;
	note?: string;
	sort: number;
	envs?: Env[];
}

interface Env {
	key: string;
	value: string;
	sort: null;
	note?: string;
}
