import { EnvironmentVariableScope } from "@/constants";
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
			id: "0",
			name: "",
			scope: EnvironmentVariableScope.USER,
			isActive: false,
			description: "",
			sort: 0,
			groups: [],
		};
	},
	variableGroup: (): VariableGroup => {
		return {
			id: "0",
			configId: "0",
			name: "",
			description: "",
			sort: 0,
			variables: [],
		};
	},
	environmentVariable: (): EnvironmentVariable => {
		return {
			id: "0",
			key: "",
			value: "",
			description: "",
			sort: 0,
		};
	},
};
