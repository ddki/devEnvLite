interface Setting {
	language: string;
	homeDir: string;
	cacheDir: string;
	dataDir: string;
	envBackupDir: string;
}

interface EnvConfig {
    id: string;
    name: string;
    scope: string;
    description?: string;
    sort?: number;
    groups?: VariableGroup[];
}

interface VariableGroup {
    id: string;
    configId: string;
    name: string;
    description?: string;
    isActive: boolean;
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