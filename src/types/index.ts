interface Setting {
	theme: 'auto' | 'light' | 'dark';
	language: string;
	homeDir: string;
	cacheDir: string;
	dataDir: string;
	logDir: string;
	envBackupDir: string;
}

interface EnvConfig {
	id: string;
	name: string;
	scope: string;
	description?: string;
	isActive: boolean;
	sort?: number;
	groups?: VariableGroup[];
}

interface VariableGroup {
	id: string;
	configId: string;
	name: string;
	description?: string;
	sort?: number;
	variables?: EnvironmentVariable[];
}

interface EnvironmentVariable {
	id: string;
	key: string;
	value: string;
	description?: string;
	sort?: number;
}

interface Res<T> {
	code: string;
	message: string;
	data: T;
}

export type { Res, Setting, EnvConfig, VariableGroup, EnvironmentVariable };
