<template>
	<div class="h-full w-full grid grid-rows-[3.5rem_1fr]">
		<div class="flex flex-row flex-2 justify-start items-center gap-2 border-b px-2">
			<ImportDialog @reload="loadSettings" />
			<EditPopover operate="new" @reload="loadSettings">
				<Button variant="outline">
					<FilePlus />
					{{ `${t("operate.new")}${t("config.text")}` }}
				</Button>
			</EditPopover>
			<ApplyConfig />
		</div>
		<ScrollArea class="h-full w-full p-2 overflow-hidden sm:mt-1 md:mt-2">
			<p class="text-secondary-foreground text-center" v-if="(!configs || configs.length < 1)">
				{{ t("config.emptyText") }}
			</p>
			<div
				:class="`grid grid-flow-col grid-cols-1 justify-between items-center hover:bg-secondary rounded-md ${item.currentSelectedClass}`"
				v-for="item in configs" :key="item.id">
				<div class="flex flex-row gap-2 h-full items-center p-2" @click="selectedConfig(configs, item)">
					<CircleCheckBig class="text-destructive" v-if="item.isActive" />
					<File v-else />
					<div class="grid grid-flow-row w-full justify-start items-center">
						<span>{{ item.name }}</span>
						<span class="text-ellipsis text-nowrap overflow-hidden text-muted-foreground text-xs">
							{{ item.description }}
						</span>
					</div>
				</div>
				<div class="grid grid-flow-col">
					<EditPopover operate="edit" :id="item.id" @reload="loadSettings">
						<Button variant="ghost" size="icon">
							<Pencil class="h-4 w-4" />
						</Button>
					</EditPopover>
					<DropdownMenu>
						<DropdownMenuTrigger>
							<Button variant="ghost" size="icon">
								<Ellipsis class="h-4 w-4" />
							</Button>
						</DropdownMenuTrigger>
						<DropdownMenuContent>
							<DropdownMenuItem>
								<Switch :model-value="item.isActive" @update:model-value="switchActive(item)" />
								<span v-if="item.isActive">{{ t("operate.status.actived") }}</span>
								<span v-else>{{ t("operate.status.inactive") }}</span>
							</DropdownMenuItem>
							<DropdownMenuItem @click="dropdownMenuDelete(item)">
								<Trash2 class="mr-2 h-4 w-4 text-destructive" />
								<span class="text-destructive">{{ t("operate.delete") }}</span>
							</DropdownMenuItem>
							<DropdownMenuItem @click="dropdownMenuExport(item)">
								<FileDown class="mr-2 h-4 w-4" />
								<span>{{ t("operate.export") }}</span>
							</DropdownMenuItem>
						</DropdownMenuContent>
					</DropdownMenu>
				</div>
			</div>
		</ScrollArea>
	</div>
</template>

<script setup lang="ts">
import { ApplyConfig, EditPopover, ImportDialog } from "@/components/config";
import { Button } from "@/components/ui/button";
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Switch } from "@/components/ui/switch";
import type { EnvConfig, Res } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import {
	CircleCheckBig,
	Ellipsis,
	File,
	FileDown,
	FilePlus,
	Pencil,
	Trash2,
} from "lucide-vue-next";
import { type Ref, getCurrentInstance, onMounted, provide, ref } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

interface ConfigData extends EnvConfig {
	// 当前选择项的样式
	currentSelectedClass?: string;
}

const model = defineModel<EnvConfig>();

const emits = defineEmits(["update:modelValue"]);
const context = getCurrentInstance();

const { t } = useI18n();

const configs = ref<ConfigData[]>([]);

// 从后端获取配置列表
const listEnvConfigs = async () => {
	return await invoke<Res<EnvConfig[]>>("list_env_configs")
		.then((res) => {
			if (res.code === "200") {
				return res.data;
			}
			toast.error(`${t("operate.query")}${t("config.text")}`, {
				description: res.message || t("message.error"),
			});
		})
		.catch((e) => {
			toast.error(`${t("operate.query")}${t("config.text")}`, {
				description: `${t("message.error")}: ${e.message}`,
			});
		});
};

// 加载配置列表，并设置第一个为选中项
const loadSettings = async () => {
	await listEnvConfigs().then((res) => {
		if (res) {
			configs.value = res;
			// 设置第一个为选中项
			if (configs.value.length > 0) {
				selectedConfig(configs.value, configs.value[0]);
			}
		}
	});
};

onMounted(() => {
	loadSettings();
});

// 设置选中项
const selectedConfig = (configs: ConfigData[], config: ConfigData) => {
	// biome-ignore lint/complexity/noForEach: <explanation>
	configs.forEach((item) => {
		if (item.id === config.id) {
			item.currentSelectedClass = "bg-secondary";
			return;
		}
		item.currentSelectedClass = "";
	});
	model.value = {
		...config,
	};
};

// 切换选中项
const switchActive = async (config: ConfigData) => {
	console.log("switchActive", config);
	await invoke<Res<void>>("update_env_config", {
		config: {
			...config,
			isActive: !config.isActive,
		},
	})
		.then(async (res) => {
			if (res.code === "200") {
				await loadSettings();
			} else {
				toast.error(t("operate.active"), {
					description: res.message || t("message.error"),
				});
			}
		})
		.catch((e) => {
			toast.error(t("operate.active"), {
				description: `${t("message.error")}: ${e.message}`,
			});
		});
};

// 删除
const dropdownMenuDelete = async (config: ConfigData) => {
	await invoke<Res<void>>("delete_env_config_transaction", { id: config.id })
		.then(async (res) => {
			if (res.code === "200") {
				await loadSettings();
				context?.appContext.config.globalProperties.$emitter.emit("reloadApp");
				toast.success(`${t("operate.delete")}${t("config.text")}`, {
					description: t("message.success"),
				});
			} else {
				toast.error(`${t("operate.delete")}${t("config.text")}`, {
					description: res.message || t("message.error"),
				});
			}
		})
		.catch((e) => {
			toast.error(`${t("operate.delete")}${t("config.text")}`, {
				description: `${t("message.error")}: ${e.message}`,
			});
		});
};

// 导出配置
const dropdownMenuExport = async (config: ConfigData) => {
	await invoke<Res<void>>("export_env_config", { id: config.id })
		.then(async (res) => {
			if (res.code === "200") {
				toast.success(`${t("operate.export")}${t("config.text")}`, {
					description: t("message.success"),
				});
			} else {
				toast.error(`${t("operate.export")}${t("config.text")}`, {
					description: res.message || t("message.error"),
				});
			}
		})
		.catch((e) => {
			toast.error(`${t("operate.export")}${t("config.text")}`, {
				description: `${t("message.error")}: ${e.message}`,
			});
		});
};

const checkConfigNameHasExists = async (
	configName: string,
	excludeConfigId: string | undefined,
	title: string,
): Promise<boolean> => {
	return (
		(await invoke<Res<boolean>>("check_config_name_exists", {
			configName: configName,
			excludeConfigId: excludeConfigId,
		})
			.then((res) => {
				if (res.code === "200") {
					if (res.data) {
						toast.warning(title, {
							description: t("config.error.nameExists"),
						});
						return true;
					}
				} else {
					toast.error(title, {
						description: `${t("message.error")} : ${res.message}`,
					});
					return true;
				}
			})
			.catch((err) => {
				toast.error(title, {
					description: `${t("message.error")} : ${err.message}`,
				});
				return true;
			})) || false
	);
};

provide<Ref<EnvConfig[]>>("envConfigs", configs);
provide("loadSettings", loadSettings);
provide("checkConfigNameHasExists", checkConfigNameHasExists);
</script>
