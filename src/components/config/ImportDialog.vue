<template>
	<Dialog v-model:open="dialogOpen">
		<DialogTrigger as-child>
			<Button variant="outline" @click="dialogOpen = true">
				<Import class="mr-2 h-6 w-6" />
				{{ t('config.import-config.text') }}
			</Button>
		</DialogTrigger>
		<DialogContent>
			<DialogHeader>
				<DialogTitle>{{ t('config.import-config.text') }}</DialogTitle>
				<DialogDescription>
					{{ t('config.import-config.description') }}
				</DialogDescription>
				<DialogClose />
			</DialogHeader>
			<Tabs default-value="system">
				<TabsList class="grid w-full grid-cols-3">
					<TabsTrigger value="system">
						{{ t('config.import-config.types.env.text') }}
					</TabsTrigger>
					<TabsTrigger value="file">
						{{ t('config.import-config.types.file.text') }}
					</TabsTrigger>
					<TabsTrigger value="url">
						{{ t('config.import-config.types.url.text') }}
					</TabsTrigger>
				</TabsList>
				<TabsContent value="system">
					<Card>
						<CardHeader class="items-center">
							<CardTitle>{{ t('config.import-config.types.env.text') }}</CardTitle>
							<CardDescription>
								{{ t('config.import-config.types.env.description') }}
							</CardDescription>
						</CardHeader>
						<CardContent class="space-y-2">
							<div class="grid grid-cols-4 items-center gap-2">
								<Label for="name" class="text-right">{{ t('config.import-config.types.env.scope') }}</Label>
								<RadioGroup v-model="systemScope">
									<div class="flex items-center space-x-2" v-for="item in scopesList" :key="item.value">
										<RadioGroupItem :value="item.value" />
										<Label for="r1">{{ item.label }}</Label>
									</div>
								</RadioGroup>
							</div>
							<div class="grid grid-cols-4 items-center gap-2">
								<Label for="name" class="text-right">{{ t('config.import-config.types.env.name') }}</Label>
								<Input v-model="systemConfigName" class="col-span-3" />
							</div>
						</CardContent>
						<CardFooter class="justify-center">
							<Button @click="importFromSystem">
								{{ t('config.import-config.text') }}
							</Button>
						</CardFooter>
					</Card>
				</TabsContent>

				<TabsContent value="file">
					<Card>
						<CardHeader class="items-center">
							<CardTitle>{{ t('config.import-config.types.file.text') }}</CardTitle>
							<CardDescription>
								{{ t('config.import-config.types.file.description') }}
							</CardDescription>
						</CardHeader>
						<CardContent class="space-y-2">
							<div class="grid grid-cols-4 items-center gap-2">
								<Label for="name" class="text-right">{{ t('config.import-config.types.file.name') }}</Label>
								<Input v-model="fileConfigName" type="text" class="col-span-3" />
							</div>
							<div class="grid grid-cols-4 items-center gap-2">
								<Label for="name" class="text-right">{{ t('config.import-config.types.file.file') }}</Label>
								<LocalFileInput v-model="filePath" type="file" :placeholder="t('config.import-config.types.file.file')"
									class="col-span-3" />
							</div>
						</CardContent>
						<CardFooter class="justify-center">
							<Button @click="importFromFile">
								{{ t('config.import-config.text') }}
							</Button>
						</CardFooter>
					</Card>
				</TabsContent>

				<TabsContent value="url">
					<Card>
						<CardHeader class="items-center">
							<CardTitle>{{ t('config.import-config.types.url.text') }}</CardTitle>
							<CardDescription>
								{{ t('config.import-config.types.url.description') }}
							</CardDescription>
						</CardHeader>
						<CardContent class="space-y-2">
							<div class="grid grid-cols-4 items-center gap-2">
								<Label for="name" class="text-right">{{ t('config.import-config.types.url.name') }}</Label>
								<Input v-model="urlConfigName" type="text" class="col-span-3" />
							</div>
							<div class="grid grid-cols-4 items-center gap-2">
								<Label for="name" class="text-right">{{ t('config.import-config.types.url.url') }}</Label>
								<Input v-model="url" type="url" class="col-span-3" />
							</div>
						</CardContent>
						<CardFooter class="justify-center">
							<Button @click="importFromUrl">
								{{ t('config.import-config.text') }}
							</Button>
						</CardFooter>
					</Card>
				</TabsContent>
			</Tabs>
			<DialogFooter>
			</DialogFooter>
		</DialogContent>
	</Dialog>
</template>

<script setup lang="ts">
import { LocalFileInput } from "@/components/common";
import { Button } from "@/components/ui/button";
import {
	Card,
	CardContent,
	CardDescription,
	CardFooter,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import {
	Dialog,
	DialogClose,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { EnvironmentVariableScope, scopesList } from "@/constants";
import type { EnvConfig, Res, VariableGroup } from "@/types";
import { validateUrl } from "@/utils/ValidateUtil";
import { invoke } from "@tauri-apps/api/core";
import { Import } from "lucide-vue-next";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

const { t } = useI18n();

const emit = defineEmits(["reload"]);

const dialogOpen = ref(false);

const systemConfigName = ref("");
const systemScope = ref("USER");
const fileConfigName = ref("");
const urlConfigName = ref("");
const filePath = ref("");
const url = ref("");

const init = () => {
	systemConfigName.value = "";
	fileConfigName.value = "";
	urlConfigName.value = "";
	filePath.value = "";
	url.value = "";
};

// 设置环境变量组
const setGroupsForEnvConfig = (config: EnvConfig, envs: Map<string, string>) => {
	const group: VariableGroup = {
		name: `${t("common.default")}-${t("envGroup.text")}`,
		sort: 1,
		variables: Array.from(envs).map(([key, value], index) => ({
			key,
			value,
			sort: index + 1,
		})),
	};
	config.groups = [...(config.groups || []), group];
};

// 创建/导入环境变量配置
const createEnvConfig = async (title: string, config: EnvConfig) => {
	await invoke<Res<string>>("create_env_config_transaction", { config })
		.then((creteRes) => {
			if (creteRes.code === "200") {
				emit("reload");
				toast.success(title, {
					description: t("message.success"),
				});
			} else {
				toast.error(title, {
					description: `${t("message.failure")} : ${creteRes.message}`,
				});
			}
		})
		.catch((err) => {
			toast.error(title, {
				description: `${t("message.error")} : ${err.message}`,
			});
		});
};

// 按钮功能：从系统环境变量导入
const importFromSystem = async () => {
	console.log("importFromSystem, systemConfigName: ", systemConfigName);
	if (!systemScope.value || systemScope.value.length < 0) {
		toast.warning(t("config.import-config.types.env.text"), {
			description: t("env.error.checkScope"),
		});
		return;
	}
	if (!systemConfigName.value || systemConfigName.value.length < 0) {
		toast.warning(t("config.import-config.types.env.text"), {
			description: t("config.error.nameNotEmpty"),
		});
		return;
	}
	await invoke<Res<Record<string, string>>>("get_os_environment_variables", {
		scope: systemScope.value,
	})
		.then(async (res) => {
			if (res.code === "200") {
				const resMap = new Map<string, string>(Object.entries(res.data));
				const config: EnvConfig = {
					scope: systemScope.value as EnvironmentVariableScope,
					name: systemConfigName.value,
					description: `${t("config.import-config.text")}-${t("config.import-config.types.env.text")}`,
					isActive: false,
					sort: 1,
				};
				// 设置变量组
				setGroupsForEnvConfig(config, resMap);
				const title = `${t("config.import-config.text")}-${t("config.import-config.types.env.text")}`;
				await createEnvConfig(title, config);
			} else {
				toast.error(t("config.import-config.types.env.text"), {
					description: `${t("message.error")} : ${res.message}`,
				});
			}
		})
		.catch((err) => {
			toast.error(t("config.import-config.types.env.text"), {
				description: `${t("message.error")} : ${err.message}`,
			});
		});
};

// 按钮功能：从文件导入
const importFromFile = async () => {
	// TODO fix
	if (!fileConfigName.value || fileConfigName.value.length < 0) {
		toast.warning(t("config.import-config.types.file.text"), {
			description: `${t("config.import-config.types.file.name")}${t("message.not-empty")}`,
		});
		return;
	}
	if (!filePath.value || filePath.value.length < 0) {
		toast.warning(t("config.import-config.types.file.text"), {
			description: `${t("config.import-config.types.file.file")}${t("message.not-empty")}`,
		});
		return;
	}

	// TODO file tranform to config
	const config = {
		scope: EnvironmentVariableScope.USER, // 从配置文件中提取
		name: "", // 从配置文件中提取
		isActive: false,
		sort: 1,
	};
	const title = `${t("config.import-config.text")}-${t("config.import-config.types.file.text")}`;
	await createEnvConfig(title, config);
};

// 按钮功能：从URL导入
const importFromUrl = async () => {
	// TODO fix
	if (!urlConfigName.value || urlConfigName.value.length < 0) {
		toast.warning(t("config.import-config.types.url.text"), {
			description: `${t("config.import-config.types.url.name")}${t("message.not-empty")}`,
		});
		return;
	}
	if (!url.value || url.value.length < 0) {
		toast.warning(t("config.import-config.types.url.text"), {
			description: `${t("config.import-config.types.url.url")}${t("message.not-empty")}`,
		});
		return;
	}
	if (!validateUrl(url.value)) {
		toast.warning(t("config.import-config.types.url.text"), {
			description: `${t("config.import-config.types.url.url")}${t("message.error-format")}`,
		});
		return;
	}

	// TODO url tranform to config
	const config = {
		scope: EnvironmentVariableScope.USER, // 从配置文件中提取
		name: "", // 从配置文件中提取
		isActive: false,
		sort: 1,
	};
	const title = `${t("config.import-config.text")}-${t("config.import-config.types.url.text")}`;
	await createEnvConfig(title, config);
};

watch(dialogOpen, (newValue) => {
	if (!newValue) {
		init();
	}
});
</script>

