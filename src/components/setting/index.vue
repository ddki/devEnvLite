<template>
	<Dialog v-model:open="open">
		<DialogTrigger as-child>
			<Button variant="ghost" size="icon">
				<Settings />
			</Button>
		</DialogTrigger>
		<DialogContent>
			<DialogHeader>
				<DialogTitle>{{ t('header.setting') }}</DialogTitle>
				<DialogDescription>
				</DialogDescription>
			</DialogHeader>
			<div class="grid gap-4 py-4">
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="language" class="text-right">
						{{ t('settings.language') }}
					</Label>
					<Select v-model="settingData.language">
						<SelectTrigger class="col-span-3">
							<SelectValue :placeholder="t('settings.language')" />
						</SelectTrigger>
						<SelectContent>
							<SelectGroup>
								<SelectItem v-for="(item, index) in languageList" :key="index" :value="item.value">
									{{ item.label }}
								</SelectItem>
							</SelectGroup>
						</SelectContent>
					</Select>
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label class="text-right">
						{{ t('settings.home-dir') }}
					</Label>
					<LocalFileInput type="folder" v-model="settingData.homeDir" :placeholder="t('settings.home-dir')"
						class="col-span-3" />
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label class="text-right">
						{{ t('settings.cache-dir') }}
					</Label>
					<LocalFileInput type="folder" v-model="settingData.cacheDir" :placeholder="t('settings.cache-dir')"
						class="col-span-3" />
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label class="text-right">
						{{ t('settings.data-dir') }}
					</Label>
					<LocalFileInput type="folder" v-model="settingData.dataDir" :placeholder="t('settings.data-dir')"
						class="col-span-3" />
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label class="text-right">
						{{ t('settings.env-backup-dir') }}
					</Label>
					<LocalFileInput type="folder" v-model="settingData.envBackupDir" :placeholder="t('settings.env-backup-dir')"
						class="col-span-3" />
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label class="text-right">
						{{ t('settings.log-dir') }}
					</Label>
					<LocalFileInput type="folder" v-model="settingData.logDir" :placeholder="t('settings.log-dir')" :disabled="true"
						class="col-span-3" />
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label class="text-right">
						{{ t('version') }}
					</Label>
					<Badge>{{ appVersion }}</Badge>
				</div>
			</div>
			<DialogFooter>
				<Button type="submit" @click="onSave">
					{{ t('operate.save') }}
				</Button>
			</DialogFooter>
		</DialogContent>
	</Dialog>
</template>

<script setup lang="ts">
import { LocalFileInput } from "@/components/common/index";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import {
	Select,
	SelectContent,
	SelectGroup,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select";
import { getSetting, saveSetting } from "@/store";
import { getVersion } from "@tauri-apps/api/app";
import { Settings } from "lucide-vue-next";
import { getCurrentInstance, reactive, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

const appVersion = await getVersion();
const { t } = useI18n();
const context = getCurrentInstance();

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
console.log("setting", setting);

const settingData = reactive({
	language: setting.language,
	homeDir: setting.homeDir,
	cacheDir: setting.cacheDir,
	dataDir: setting.dataDir,
	logDir: setting.logDir,
	envBackupDir: setting.envBackupDir,
});
const open = ref(false);

const onSave = async () => {
	const save = await saveSetting({
		language: settingData.language,
		homeDir: settingData.homeDir,
		cacheDir: settingData.cacheDir,
		dataDir: settingData.dataDir,
		logDir: settingData.logDir,
		envBackupDir: settingData.envBackupDir,
	});
	if (save) {
		open.value = false;
		context?.appContext.config.globalProperties.$emitter.emit("reloadApp");
	} else {
		toast({
			title: t("header.setting"),
			description: t("message.operate-failure", { operate: t("operate.save") }),
			variant: "destructive",
		});
	}
};

watch(open, async (newValue) => {
	if (!newValue) {
		const setting = await getSetting();
		settingData.language = setting.language;
		settingData.homeDir = setting.homeDir;
		settingData.cacheDir = setting.cacheDir;
		settingData.dataDir = setting.dataDir;
		settingData.envBackupDir = setting.envBackupDir;
	}
});
</script>
