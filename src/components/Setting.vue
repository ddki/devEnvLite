<template>
	<select data-choose-theme class="select select-primary w-full max-w-xs" v-model="theme">
		<option disabled selected>{{ t('setting.theme') }}</option>
		<option v-for="(item, index) in themeList" :key="index" :value="item">{{ item }}</option>
	</select>
	<select class="select select-primary w-full max-w-xs" v-model="language">
		<option disabled selected>{{ t('setting.language') }}</option>
		<option v-for="(item, index) in languageList" :key="index" :value="item.value">{{ item.label }}</option>
	</select>

	<input type="file" webkitdirectory="true" class="file-input file-input-bordered file-input-primary w-full max-w-xs" />
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import daisyuiThemes from "daisyui/src/theming/themes";
import { themeChange } from "theme-change";
import { getSetting, saveSetting } from "../store/setting";

const { t } = useI18n();

const themeList = ref(Object.keys(daisyuiThemes));
const languageList = [
	{
		value: "zh",
		label: "简体中文",
	},
	{
		value: "en",
		label: "English",
	},
];

const setting = await getSetting();

const language = ref(setting.language);
const theme = ref(setting.theme);
const dataDir = ref(setting.dataDir);
const envBackupDir = ref(setting.envBackupDir);

const onSave = async () => {
	const save = await saveSetting({
		language: language.value,
		theme: theme.value,
		dataDir: dataDir.value,
		envBackupDir: envBackupDir.value,
	});
};
</script>
