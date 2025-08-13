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
					<Label for="theme" class="text-right">
						{{ t('settings.theme.text') }}.
					</Label>
					<Select v-model="settingData.theme">
						<SelectTrigger class="col-span-3">
							<SelectValue :placeholder="t('settings.theme.text')" />
						</SelectTrigger>
						<SelectContent>
							<SelectGroup>
								<SelectItem v-for="(item, index) in themeList" :key="index" :value="item.value">
									{{ item.label }}
								</SelectItem>
							</SelectGroup>
						</SelectContent>
					</Select>
				</div>
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
import { getThemeList, languageList } from "@/constants";
import type { Res, Setting } from "@/types";
import { DefaultValue } from "@/types/defaultValue";
import { getVersion, setTheme } from "@tauri-apps/api/app";
import { invoke } from "@tauri-apps/api/core";
import { useColorMode } from "@vueuse/core";
import { Settings } from "lucide-vue-next";
import { getCurrentInstance, reactive, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

const appVersion = await getVersion();
const { t, locale } = useI18n();
const themeList = getThemeList(t);
const context = getCurrentInstance();

/**
 * 切换主题
 * @param changeTheme 切换主题
 */
const setAppTheme = async (changeTheme: string) => {
	if (changeTheme.includes("light")) {
		setTheme("light");
		theme.value = "light";
	} else if (changeTheme.includes("dark")) {
		setTheme("dark");
		theme.value = "dark";
	} else {
		setTheme(null);
		theme.value = "auto";
	}
};

const theme = useColorMode();

const setting =
	(await invoke<Res<Setting>>("get_settings")
		.then((res) => {
			if (res.code === "200") {
				return res.data;
			}
		})
		.catch(() => {
			toast.warning(t("header.setting"), {
				description: `${t("operate.save")}${t("message.failure")}`,
			});
		})) || DefaultValue.setting();

console.log("setting", setting);
// 初始化获取配置主题
await setAppTheme(setting.theme);

const settingData = reactive<Setting>({
	...setting,
});
const open = ref(false);

const onSave = async () => {
	await invoke<Res<boolean>>("save_settings", {
		settings: settingData,
	})
		.then(async (res) => {
			if (res.code === "200") {
				await setAppTheme(settingData.theme);
				locale.value = settingData.language;
				open.value = false;
				context?.appContext.config.globalProperties.$emitter.emit("reloadApp");
			}
		})
		.catch(() => {
			toast.error(t("header.setting"), {
				description: `${t("operate.save")}${t("message.failure")}`,
			});
		});
};

watch(open, async (newValue) => {
	if (!newValue) {
		Object.assign(settingData, setting);
	}
});
</script>
