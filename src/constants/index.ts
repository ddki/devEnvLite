import { useI18n } from "vue-i18n";

const { t } = useI18n();

enum EnvironmentVariableScope {
	USER = "USER",
	SYSTEM = "SYSTEM",
}

const environmentVariableScopeList = [
	{ label: t("env.scopes.user"), value: EnvironmentVariableScope.USER },
	{ label: t("env.scopes.system"), value: EnvironmentVariableScope.SYSTEM },
];

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

const themeList = [
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

export {
	environmentVariableScopeList as scopesList,
	languageList,
	themeList,
	EnvironmentVariableScope,
};
