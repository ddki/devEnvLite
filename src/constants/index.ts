enum EnvironmentVariableScope {
	USER = "user",
	SYSTEM = "system",
}

const getEnvironmentVariableScopeList = (t: (key: string) => string) => {
	return [
		{ label: t("env.scopes.user"), value: EnvironmentVariableScope.USER },
		{ label: t("env.scopes.system"), value: EnvironmentVariableScope.SYSTEM },
	];
};

const languageList = [
	{
		value: "zh-CN",
		label: "简体中文",
	},
	{
		value: "en-US",
		label: "English",
	},
];

const getThemeList = (t: (key: string) => string) => {
	return [
		{
			value: "auto",
			label: t("settings.theme.auto"),
		},
		{
			value: "light",
			label: t("settings.theme.light"),
		},
		{
			value: "dark",
			label: t("settings.theme.dark"),
		},
	];
};

export { getEnvironmentVariableScopeList, languageList, getThemeList, EnvironmentVariableScope };
