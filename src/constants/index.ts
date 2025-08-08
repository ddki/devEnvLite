import { useI18n } from "vue-i18n";

enum EnvironmentVariableScope {
	USER = "user",
	SYSTEM = "system",
}

const getEnvironmentVariableScopeList = () => {
	const { t } = useI18n();
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

const getThemeList = () => {
	const { t } = useI18n();
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
