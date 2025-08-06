import type { EnvConfig, EnvironmentVariable, Setting, VariableGroup } from ".";

export const DefaultValue = {
	setting: (): Setting => {
		return {
			theme: "auto",
			language: "zh-CN",
			homeDir: "",
			cacheDir: "",
			dataDir: "",
			logDir: "",
			envBackupDir: "",
		};
	},
	envConfig: (): EnvConfig => {
		return {
			id: undefined,
			name: "",
			scope: "USER",
			isActive: false,
			description: "",
			sort: 0,
			groups: [],
		};
	},
	variableGroup: (): VariableGroup => {
		return {
			id: undefined,
			configId: undefined,
			name: "",
			description: "",
			sort: 0,
			variables: [],
		};
	},
	environmentVariable: (): EnvironmentVariable => {
		return {
			id: undefined,
			key: "",
			value: "",
			description: "",
			sort: 0,
		};
	},
};
