<template>
	<div class="h-full w-full grid grid-rows-[3.5rem_1fr]">
		<div class="flex flex-row flex-2 justify-start items-center gap-2 border-b px-2">
			<ImportDialog @callback="loadStore" />
			<EditPopover operate="new" @callback="loadStore">
				<Button variant="outline">
					<FilePlus class="mr-2 h-6 w-6" />
					{{ t("operate.new", { name: t("config.text") }) }}
				</Button>
			</EditPopover>
		</div>
		<ScrollArea class="h-full w-full p-2">
			<div class="sm:mt-1 md:mt-2 overflow-auto">
				<p class="text-secondary-foreground text-center" v-if="(!configs || configs.length < 1)">
					{{ t("config.emptyText") }}
				</p>
				<div
					:class="`grid grid-flow-col grid-cols-1 justify-between items-center hover:bg-secondary rounded-md ${item.activeClass}`"
					v-for="item in configs">
					<div class="flex flex-row gap-2 h-full items-center p-2" @click="onClickConfig(item)">
						<CircleCheckBig class="text-destructive" v-if="item.isActive" />
						<File />
						<div class="grid grid-flow-row w-full justify-start items-center">
							<span>{{ item.name }}</span>
							<span class="text-ellipsis text-nowrap overflow-hidden text-muted-foreground text-xs">
								{{ item.note }}
							</span>
						</div>
					</div>
					<div class="grid grid-flow-col">
						<EditPopover operate="edit" :id="item.id" @callback="loadStore">
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
								<DropdownMenuItem @click="dropdownMenuCheck(item)">
									<SearchCheck class="mr-2 h-4 w-4" />
									<span>{{ t("operate.check") }}</span>
								</DropdownMenuItem>
								<DropdownMenuItem @click="dropdownMenuApply(item)">
									<CircleCheck class="mr-2 h-4 w-4" />
									<span>{{ t("operate.apply") }}</span>
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
import { useToast } from "@/components/ui/toast";
import { deleteConfig, getConfigs, setActiveConfigId } from "@/store";
import type { Config } from "@/store/type";
import { invoke } from "@tauri-apps/api/core";
import {
	CircleCheckBig,
	Ellipsis,
	File,
	FilePlus,
	CircleCheck,
	Pencil,
	SearchCheck,
	Trash2,
	FileDown,
} from "lucide-vue-next";
import { getCurrentInstance, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

interface ConfigData extends Config {
	activeClass?: string;
	isActive?: boolean;
}

const props = defineProps({
	activeConfigId: String,
	selectedConfigId: String,
});
const emits = defineEmits(["update:activeConfigId", "update:selectedConfigId"]);
const context = getCurrentInstance();

const { t } = useI18n();
const { toast } = useToast();

const configs = ref<ConfigData[]>([]);

const loadStore = async () => {
	const configIds = (await invoke("get_config_ids")) as string[];
	const storeConfigs = (await getConfigs(configIds))
		.filter((item) => item.id && item.name)
		.map((item) => item as ConfigData)
		.map((item) => {
			if (item.id === props.activeConfigId) {
				item.isActive = true;
			}
			if (item.id === props.selectedConfigId) {
				item.activeClass = "bg-secondary";
			}
			return item;
		})
		.sort((a, b) => {
			if (a.isActive === b.isActive) {
				return a.sort - b.sort;
			}
			return a.isActive ? -1 : 1;
		});
	configs.value = storeConfigs;
};

await loadStore();

const resetConfigsActiveClass = () => {
	configs.value = configs.value.map((item) => {
		item.activeClass = "";
		return item;
	});
};

const onClickConfig = (config: ConfigData) => {
	resetConfigsActiveClass();
	config.activeClass = "bg-secondary";
	emits("update:selectedConfigId", config.id);
};

// 激活
const dropdownMenuActive = async (config: ConfigData) => {
	await setActiveConfigId(config.id);
	emits("update:activeConfigId", config.id);
};

// 检查
const dropdownMenuCheck = async (config: ConfigData) => {
	await invoke("config_check", { configId: config.id })
		.then(async () => {
			await loadStore();
			context?.appContext.config.globalProperties.$emitter.emit("reloadApp");
			toast({
				title: `${t("operate.check")} ${t("config.text")}`,
				description: t("message.success"),
			});
		})
		.catch((e) => {
			toast({
				title: `${t("operate.check")} ${t("config.text")}`,
				description: `${t("message.error")}: ${e.message}`,
				variant: "destructive",
			});
			console.log("application startup config_check error: ", e);
		});
};

// 应用
const dropdownMenuApply = async (config: ConfigData) => {
	await invoke("config_apply", { configId: config.id })
		.then(async () => {
			await loadStore();
			context?.appContext.config.globalProperties.$emitter.emit("reloadApp");
			toast({
				title: `${t("operate.apply")} ${t("config.text")}`,
				description: t("message.success"),
			});
		})
		.catch((e) => {
			toast({
				title: `${t("operate.apply")} ${t("config.text")}`,
				description: `${t("message.error")}: ${e.message}`,
				variant: "destructive",
			});
			console.log("application startup config_check error: ", e);
		});
};

// 删除
const dropdownMenuDelete = async (config: ConfigData) => {
	await deleteConfig(config.id)
		.then(async () => {
			await loadStore();
			context?.appContext.config.globalProperties.$emitter.emit("reloadApp");
			toast({
				title: `${t("operate.delete", { name: t("config.text") })}`,
				description: t("message.success"),
			});
		})
		.catch((e) => {
			toast({
				title: `${t("operate.delete", { name: t("config.text") })}`,
				description: `${t("message.error")}: ${e.message}`,
				variant: "destructive",
			});
			console.log("application startup config_check error: ", e);
		});
};

// 导出配置
const dropdownMenuExport = async (config: ConfigData) => {
	await invoke("config_export", { configId: config.id })
		.then(async () => {
			toast({
				title: `${t("operate.export", { name: t("config.text") })}`,
				description: t("message.success"),
			});
		})
		.catch((e) => {
			toast({
				title: `${t("operate.export", { name: t("config.text") })}`,
				description: `${t("message.error")}: ${e.message}`,
				variant: "destructive",
			});
		});
}

watch(
	() => props.activeConfigId,
	async (newValue, oldValue) => {
		if (newValue && newValue !== oldValue) {
			await loadStore();
		}
	},
);
</script>
