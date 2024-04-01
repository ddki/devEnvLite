<template>
	<el-dialog v-model="props.visible" :title="props.title" @close="closeDialog" width="32rem" v-if="props.visible">
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
				<Button @click="onSave">
					{{ t('save') }}
				</Button>
			</div>
		</template>
	</el-dialog>
</template>

<script setup lang="ts">
import { getVersion } from "@tauri-apps/api/app";
import { reactive } from "vue";
import { useI18n } from "vue-i18n";
import Button from "@/components/ui/button/Button.vue";
import { useToast } from '@/components/ui/toast/use-toast'
import { getSetting, saveSetting } from "@/store/setting";


const props = defineProps({
	title: String,
	visible: {
		type: Boolean,
		default: false,
	},
});

const appVersion = await getVersion();
const { t } = useI18n();
const { toast } = useToast()

const emits = defineEmits(["callBack", "update:visible"]);

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
		emits("callBack");
		toast({
			title: t("header.setting"),
			description: t("save") + t("success")
		});
	} else {
		toast({
			title: t("header.setting"),
			description: t("save") + t("failure")
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
