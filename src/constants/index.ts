import { useI18n } from "vue-i18n";

const { t } = useI18n();

const scopesList = [
	{ label: t("env.scopes.user"), value: "USER" },
	{ label: t("env.scopes.system"), value: "SYSTEM" },
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


export { scopesList, languageList, themeList };
