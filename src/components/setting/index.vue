<template>
	<Dialog v-model:open="open">
		<DialogTrigger as-child>
			<Button variant="ghost" size="icon">
				<Settings />
			</Button>
		</DialogTrigger>
		<DialogContent class="sm:max-w-[425px]">
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
					<Input v-model="settingData.homeDir" class="col-span-3" />
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label class="text-right">
						{{ t('settings.cache-dir') }}
					</Label>
					<Input v-model="settingData.cacheDir" class="col-span-3" />
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label class="text-right">
						{{ t('settings.data-dir') }}
					</Label>
					<Input v-model="settingData.dataDir" class="col-span-3" />
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label class="text-right">
						{{ t('settings.env-backup-dir') }}
					</Label>
					<Input v-model="settingData.envBackupDir" class="col-span-3" />
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
					{{ t('save') }}
				</Button>
			</DialogFooter>
		</DialogContent>
	</Dialog>
</template>

<script setup lang="ts">
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
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
	Select,
	SelectContent,
	SelectGroup,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select";
import { Badge } from "@/components/ui/badge";
import { useToast } from "@/components/ui/toast/use-toast";
import { getSetting, saveSetting } from "@/store";
import { getVersion } from "@tauri-apps/api/app";
import { Settings } from "lucide-vue-next";
import { reactive, ref } from "vue";
import { useI18n } from "vue-i18n";

const appVersion = await getVersion();
const { t } = useI18n();
const { toast } = useToast();

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

const settingData = reactive({
	language: setting.language,
	homeDir: setting.homeDir,
	cacheDir: setting.cacheDir,
	dataDir: setting.dataDir,
	envBackupDir: setting.envBackupDir,
});
const open = ref(false);

const onSave = async () => {
	const save = await saveSetting({
		language: settingData.language,
		homeDir: settingData.homeDir,
		cacheDir: settingData.cacheDir,
		dataDir: settingData.dataDir,
		envBackupDir: settingData.envBackupDir,
	});
	if (save) {
		open.value = false;
	} else {
		toast({
			title: t("header.setting"),
			description: t("save") + t("failure"),
			variant: "destructive",
		});
	}
};
</script>
