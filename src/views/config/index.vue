<template>
	<div class="h-full w-full grid grid-rows-[3.5rem_1fr]">
		<div class="flex flex-row flex-2 justify-start items-center gap-2 border-b px-2">
			<ImportDialog @reload="loadSettings" />
			<EditPopover operate="new" @reload="loadSettings">
				<Button variant="outline">
					<FilePlus class="mr-2 h-6 w-6" />
					{{ `${t("operate.new")}${t("config.text")}` }}
				</Button>
			</EditPopover>
		</div>
		<ScrollArea class="h-full w-full p-2">
			<div class="sm:mt-1 md:mt-2 overflow-auto">
				<p class="text-secondary-foreground text-center" v-if="(!configs || configs.length < 1)">
					{{ t("config.emptyText") }}
				</p>
				<div
					:class="`grid grid-flow-col grid-cols-1 justify-between items-center hover:bg-secondary rounded-md ${item.currentSelectedClass}`"
					v-for="item in configs">
					<div class="flex flex-row gap-2 h-full items-center p-2" @click="selectedConfig(item)">
						<CircleCheckBig class="text-destructive" v-if="item.isActive" />
						<File />
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
								<DropdownMenuItem @click="dropdownMenuActive(item)">
									<CircleCheckBig class="mr-2 h-4 w-4" />
									<span>{{ t("operate.active") }}</span>
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
			</div>
		</ScrollArea>
	</div>
</template>

<script setup lang="ts">
import { EditPopover, ImportDialog } from "@/components/config";
import { Button } from "@/components/ui/button";
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { ScrollArea } from "@/components/ui/scroll-area";
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
import { getCurrentInstance, provide, ref } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

interface ConfigData extends EnvConfig {
	// 当前选择项的样式
	currentSelectedClass?: string;
}

const props = defineProps({
	currentConfigId: String,
});
const emits = defineEmits(["update:currentConfigId"]);
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
				selectedConfig(configs.value[0]);
			}
		}
	});
};

// 设置选中项
const selectedConfig = (config: ConfigData) => {
	config.currentSelectedClass = "bg-secondary";
	emits("update:currentConfigId", config.id);
};

// 激活
const dropdownMenuActive = async (config: ConfigData) => {
	await invoke<Res<void>>("update_env_config", { configId: config.id, isActive: true })
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
	await invoke<Res<void>>("delete_env_config", { configId: config.id })
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

// TODO 导出配置
const dropdownMenuExport = async (config: ConfigData) => {
	await invoke<Res<void>>("export_env_config", { configId: config.id })
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

provide("listEnvConfigs", listEnvConfigs);
provide("loadSettings", loadSettings);
</script>
