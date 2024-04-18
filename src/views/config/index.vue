<template>
	<div class="h-full w-full grid grid-rows-[3.5rem_1fr]">
		<div class="flex flex-row flex-2 justify-start items-center gap-2 border-b px-2">
			<ImportDialog @callback="loadStore" />
			<EditPopover operate="new" @callback="loadStore">
				<Button variant="outline">
					<FilePlus class="mr-2 h-6 w-6" />
					{{ t("config.new-config") }}
				</Button>
			</EditPopover>
		</div>
		<ScrollArea class="h-full w-full p-2">
			<div class="sm:mt-1 md:mt-2 overflow-auto">
				<span class="text-secondary" v-if="(!configs || configs.length < 1)">{{ t("config.emptyText") }}</span>
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
									<span>{{ t("config.context-menu.active") }}</span>
								</DropdownMenuItem>
								<DropdownMenuItem @click="dropdownMenuCheck(item)">
									<SearchCheck class="mr-2 h-4 w-4" />
									<span>{{ t("config.context-menu.check") }}</span>
								</DropdownMenuItem>
								<DropdownMenuItem @click="dropdownMenuApply(item)">
									<Laugh class="mr-2 h-4 w-4" />
									<span>{{ t("config.context-menu.apply") }}</span>
								</DropdownMenuItem>
								<DropdownMenuItem @click="dropdownMenuDelete(item)">
									<Trash2 class="mr-2 h-4 w-4 text-destructive" />
									<span class="text-destructive">{{ t("config.context-menu.delete") }}</span>
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
import { deleteConfig, getConfigs, setActiveConfigId } from "@/store/config";
import type { Config } from "@/store/type";
import { invoke } from "@tauri-apps/api/core";
import {
	CircleCheckBig,
	Ellipsis,
	File,
	FilePlus,
	Laugh,
	Pencil,
	SearchCheck,
	Trash2,
} from "lucide-vue-next";
import { ref, watch } from "vue";
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

const { t } = useI18n();

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
	// todo
	await invoke("config_check");
};

// 应用
const dropdownMenuApply = async (config: ConfigData) => {
	// todo
	await invoke("config_apply");
};

// 删除
const dropdownMenuDelete = async (config: ConfigData) => {
	await deleteConfig(config.id);
	await loadStore();
};

watch(
	() => props.activeConfigId,
	async (newValue, oldValue) => {
		if (newValue !== oldValue) {
			await loadStore();
		}
	},
);
</script>
