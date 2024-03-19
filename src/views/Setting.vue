<template>
	<div class="item">
		<span>
			{{ t('settings.theme') }}
		</span>
		<select data-choose-theme class="select select-primary w-full max-w-xs" v-model="settingForm.theme">
			<option v-for="(item, index) in themeList" :key="index" :value="item">
				{{ item }}
			</option>
		</select>
	</div>
	<div class="item">
		<span>
			{{ t('settings.language') }}
		</span>
		<select class="select select-primary w-full max-w-xs" v-model="settingForm.language">
			<option v-for="(item, index) in languageList" :key="index" :value="item.value">
				{{ item.label }}
			</option>
		</select>
	</div>
	<div class="item">
		<span>
			{{ t('settings.home-dir') }}
		</span>
		<input class="input input-bordered input-primary w-full max-w-xs" v-model="settingForm.homeDir"
			:placeholder="t('settings.home-dir')" />
	</div>
	<div class="item">
		<span>
			{{ t('settings.cache-dir') }}
		</span>
		<input class="input input-bordered input-primary w-full max-w-xs" v-model="settingForm.cacheDir"
			:placeholder="t('settings.cache-dir')" />
	</div>
	<div class="item">
		<span>
			{{ t('settings.data-dir') }}
		</span>
		<input class="input input-bordered input-primary w-full max-w-xs" v-model="settingForm.dataDir"
			:placeholder="t('settings.data-dir')" />
	</div>
	<div class="item">
		<span>
			{{ t('settings.env-backup-dir') }}
		</span>
		<input class="input input-bordered input-primary w-full max-w-xs" v-model="settingForm.envBackupDir"
			:placeholder="t('settings.env-backup-dir')" />
	</div>

	<div class="flex flex-row justify-end mt-8">
		<button type="button" class="btn btn-primary btn-sm sm:btn-sm md:btn-md" @click="onSave">
			{{ t('save') }}
		</button>
	</div>

	<el-dialog v-model="dialogFormVisible" title="Shipping address" width="500">
		<el-form :model="settingForm">
			<el-form-item :label="t('settings.language')">
				<el-select v-model="settingForm.language" :placeholder="t('settings.language')">
					<option v-for="(item, index) in languageList" :key="index" :value="item.value">
						{{ item.label }}
					</option>
				</el-select>
			</el-form-item>
			<el-form-item :label="t('settings.home-dir')">
				<el-input v-model="settingForm.homeDir" />
			</el-form-item>
			<el-form-item :label="t('settings.cache-dir')">
				<el-input v-model="settingForm.cacheDir" />
			</el-form-item>
			<el-form-item :label="t('settings.data-dir')">
				<el-input v-model="settingForm.dataDir" />
			</el-form-item>
			<el-form-item :label="t('settings.env-backup-dir')">
				<el-input v-model="settingForm.envBackupDir" />
			</el-form-item>
		</el-form>
		<template #footer>
			<div class="dialog-footer">
				<el-button type="primary" @click="dialogFormVisible = false">
					Confirm
				</el-button>
			</div>
		</template>
	</el-dialog>
</template>

<script setup lang="ts">
import { getCurrentInstance, reactive } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessage } from 'element-plus'
import { useDark, useToggle } from '@vueuse/core'
import { getSetting, saveSetting } from "../store/setting";


const { t } = useI18n();
const global = getCurrentInstance()?.appContext.config.globalProperties;

const emits = defineEmits(["callBack"]);
const isDark = useDark()

const themeList = ["light", "dark"];
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

const settingForm = reactive({
	language: setting.language,
	theme: setting.theme,
	homeDir: setting.homeDir,
	cacheDir: setting.cacheDir,
	dataDir: setting.dataDir,
	envBackupDir: setting.envBackupDir,
});

const onSave = async () => {
	const save = await saveSetting({
		language: settingForm.language,
		theme: settingForm.theme,
		homeDir: settingForm.homeDir,
		cacheDir: settingForm.cacheDir,
		dataDir: settingForm.dataDir,
		envBackupDir: settingForm.envBackupDir,
	});
	if (save) {
		useToggle(isDark);
		emits("callBack");
	} else {
		ElMessage.error(t("save") + t("failure"))
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
	padding: 0.2rem 0;
}
</style>
