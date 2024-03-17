interface Setting {
	language: string | null;
	theme: string | null;
	homeDir: string | null;
	cacheDir: string | null;
	dataDir: string | null;
	envBackupDir: string | null;
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
	envs: Env[];
}

interface Env {
	key: string;
	value: string;
	sort: null;
	note?: string;
}
