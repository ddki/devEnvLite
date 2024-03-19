<template>
	<el-dialog v-model="props.visible" :title="props.title" @close="closeDialog" width="32rem">
		<el-form :model="settingForm" label-position="right" label-width="auto">
			<el-form-item :label="t('settings.language')">
				<el-select v-model="settingForm.language" :placeholder="t('settings.language')">
					<el-option v-for="(item, index) in languageList" :key="index" :value="item.value" :label="item.label" />
				</el-select>
			</el-form-item>
			<el-form-item :label="t('settings.home-dir')">
				<el-input v-model="settingForm.homeDir" clearable :placeholder="t('settings.home-dir')" />
			</el-form-item>
			<el-form-item :label="t('settings.cache-dir')">
				<el-input v-model="settingForm.cacheDir" clearable :placeholder="t('settings.cache-dir')" />
			</el-form-item>
			<el-form-item :label="t('settings.data-dir')">
				<el-input v-model="settingForm.dataDir" clearable :placeholder="t('settings.data-dir')" />
			</el-form-item>
			<el-form-item :label="t('settings.env-backup-dir')">
				<el-input v-model="settingForm.envBackupDir" clearable :placeholder="t('settings.env-backup-dir')" />
			</el-form-item>
			<el-form-item :label="t('version')">
				<el-tag type="primary">{{ appVersion }}</el-tag>
			</el-form-item>
		</el-form>
		<template #footer>
			<div class="dialog-footer">
				<el-button type="primary" @click="onSave">
					{{ t('save') }}
				</el-button>
			</div>
		</template>
	</el-dialog>
</template>

<script setup lang="ts">
import { getVersion } from "@tauri-apps/api/app";
import { ElNotification } from "element-plus";
import { reactive } from "vue";
import { useI18n } from "vue-i18n";
import { getSetting, saveSetting } from "../../store/setting";

const props = defineProps({
	title: String,
	visible: {
		type: Boolean,
		default: false,
	},
});

const appVersion = await getVersion();
const { t } = useI18n();

const emits = defineEmits(["update:visible"]);

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
	homeDir: setting.homeDir,
	cacheDir: setting.cacheDir,
	dataDir: setting.dataDir,
	envBackupDir: setting.envBackupDir,
});

const onSave = async () => {
	const save = await saveSetting({
		language: settingForm.language,
		homeDir: settingForm.homeDir,
		cacheDir: settingForm.cacheDir,
		dataDir: settingForm.dataDir,
		envBackupDir: settingForm.envBackupDir,
	});
	if (save) {
		emits("update:visible", false);
		ElNotification({
			title: t("header.setting"),
			message: t("save") + t("success"),
			position: "bottom-right",
			type: "success",
		});
	} else {
		ElNotification({
			title: t("header.setting"),
			message: t("save") + t("failure"),
			position: "bottom-right",
			type: "error",
		});
	}
};

const closeDialog = () => {
	emits("update:visible", false);
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
../../store/setting
