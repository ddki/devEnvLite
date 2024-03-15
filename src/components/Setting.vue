<template>
	<div class="item">
		<span>
			{{ t('settings.theme') }}
		</span>
		<select data-choose-theme class="select select-primary w-full max-w-xs" v-model="settingDate.theme">
			<option disabled selected>
				{{ t('settings.theme') }}
			</option>
			<option v-for="(item, index) in themeList" :key="index" :value="item">
				{{ item }}
			</option>
		</select>
	</div>
	<div class="item">
		<span>
			{{ t('settings.language') }}
		</span>
		<select class="select select-primary w-full max-w-xs" v-model="settingDate.language">
			<option disabled selected>
				{{ t('settings.language') }}
			</option>
			<option v-for="(item, index) in languageList" :key="index" :value="item.value">
				{{ item.label }}
			</option>
		</select>
	</div>
	<div class="item">
		<span>
			{{ t('settings.home-dir') }}
		</span>
		<input class="input input-bordered input-primary w-full max-w-xs" v-model="settingDate.homeDir"
			:placeholder="t('settings.home-dir')" />
	</div>
	<div class="item">
		<span>
			{{ t('settings.cache-dir') }}
		</span>
		<input class="input input-bordered input-primary w-full max-w-xs" v-model="settingDate.cacheDir"
			:placeholder="t('settings.cache-dir')" />
	</div>
	<div class="item">
		<span>
			{{ t('settings.data-dir') }}
		</span>
		<input class="input input-bordered input-primary w-full max-w-xs" v-model="settingDate.dataDir"
			:placeholder="t('settings.data-dir')" />
	</div>
	<div class="item">
		<span>
			{{ t('settings.env-backup-dir') }}
		</span>
		<input class="input input-bordered input-primary w-full max-w-xs" v-model="settingDate.envBackupDir"
			:placeholder="t('settings.env-backup-dir')" />
	</div>

	<div class="flex flex-row justify-end mt-8">
		<button type="button" class="btn btn-primary btn-sm sm:btn-sm md:btn-md" @click="onSave">
			{{ t('save') }}
		</button>
	</div>

</template>

<script setup lang="ts">
import { themeChange } from "theme-change";
import { reactive } from "vue";
import { useI18n } from "vue-i18n";
import { getSetting, saveSetting } from "../store/setting";

const { t } = useI18n();

const themeList = ["light", "dark", "cupcake", "synthwave", "dracula", "business", "dim"];
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

const settingDate = reactive({
	language: setting.language,
	theme: setting.theme,
	homeDir: setting.homeDir,
	cacheDir: setting.cacheDir,
	dataDir: setting.dataDir,
	envBackupDir: setting.envBackupDir,
});

const onSave = async () => {
	const save = await saveSetting({
		language: settingDate.language,
		theme: settingDate.theme,
		homeDir: settingDate.homeDir,
		cacheDir: settingDate.cacheDir,
		dataDir: settingDate.dataDir,
		envBackupDir: settingDate.envBackupDir,
	});
	if (save) {
		themeChange(false);
	}
};
</script>

<style scoped>
.item {
	display: grid;
	grid-auto-columns: 1fr;
	grid-template-columns: 1fr 2fr;
	gap: 0em 1rem;
	justify-content: center;
	align-content: center;
	justify-items: end;
	align-items: center;
}
</style>
