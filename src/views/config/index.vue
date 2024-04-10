<template>
	<div class="grid sm:grid-rows-[2rem_minmax(0,_1fr)] md:grid-rows-[3rem_minmax(0,_1fr)] overflow-auto">
		<div class="flex flex-row flex-2 justify-start items-center gap-2">
			<Button variant="outline" @click="importConfig">{{ t('config.import-config') }}</Button>
			<EditPopover :triggerText="t('config.new-config')" operate="new" @callback="loadStore" />
		</div>
		<div class="sm:mt-1 md:mt-2 overflow-auto">
			<span class="text-slate-500" v-if="(!configs || configs.length < 1)">{{ t("config.emptyText") }}</span>
			<div class="grid grid-cols-[1fr_3rem] items-center" v-for="item in configs" :key="item?.id">
				<div class="flex flex-row gap-2 h-full items-center hover:bg-secondary" @click="onClickConfig(item)"
					@contextmenu="onContextMenu($event, item)">
					<CircleCheckBig v-if="item.isActive" />
					<span>{{ item?.name }}</span>
				</div>
				<DropdownMenu>
					<DropdownMenuTrigger>
						<Ellipsis />
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
						<DropdownMenuSeparator />
						<DropdownMenuItem>
							<Pencil class="mr-2 h-4 w-4" />
							<EditPopover :triggerText="t('config.context-menu.modify')" operate="edit" :id="item.id" @callback="loadStore" />
						</DropdownMenuItem>
						<DropdownMenuItem @click="dropdownMenuDelete(item)">
							<Trash2 class="mr-2 h-4 w-4" />
							<span>{{ t("config.context-menu.delete") }}</span>
						</DropdownMenuItem>
					</DropdownMenuContent>
				</DropdownMenu>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { deleteConfig, getConfigs, setActiveConfigId } from "@/store/config";
import ContextMenu from "@imengyu/vue3-context-menu";
import { invoke } from "@tauri-apps/api/core";
import { CircleCheckBig, Ellipsis, SearchCheck, Laugh, Pencil, Trash2 } from "lucide-vue-next";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import EditPopover from "@/components/config/EditPopover.vue";

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
const configId = ref("");

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
				item.activeClass = "active";
			}
			return item;
		})
		.sort((a, b) => {
			if (a.isActive === b.isActive) {
				return b.sort - a.sort;
			}
			return a.isActive ? -1 : 1;
		});
	configs.value = storeConfigs;
};

await loadStore();

const importConfig = async () => { };

const editConfig = (config: ConfigData) => {
	console.log("edit config: ", config);
	resetConfigsActiveClass();
	config.activeClass = "active";
	configId.value = config.id;
};

const resetConfigsActiveClass = () => {
	configs.value = configs.value.map((item) => {
		item.activeClass = "";
		return item;
	});
};

const onClickConfig = (config: ConfigData) => {
	resetConfigsActiveClass();
	config.activeClass = "active";
	emits("update:selectedConfigId", config.id);
};

// 激活
const dropdownMenuActive = async (config: ConfigData) => {
	await setActiveConfigId(config.id);
	emits("update:activeConfigId", config.id);
}

// 检查
const dropdownMenuCheck = (config: ConfigData) => {

}

// 应用
const dropdownMenuApply = (config: ConfigData) => {

}

// 删除
const dropdownMenuDelete = async (config: ConfigData) => {
	await deleteConfig(config.id);
	await loadStore();
}

// 右键菜单
const onContextMenu = (e: MouseEvent, config: ConfigData) => {
	e.preventDefault();
	ContextMenu.showContextMenu({
		x: e.x,
		y: e.y,
		items: [
			{
				label: t("config.context-menu.active"),
				onClick: async () => {
					await setActiveConfigId(config.id);
					emits("update:activeConfigId", config.id);
				},
				hidden: config.id === props.activeConfigId,
			},
			{
				label: t("config.context-menu.check"),
				onClick: () => {
					// todo 调用rust
				},
			},
			{
				label: t("config.context-menu.apply"),
				onClick: () => {
					// todo 调用rust
				},
			},
			{
				label: t("config.context-menu.modify"),
				onClick: () => {
					editConfig(config);
				},
			},
			{
				label: t("config.context-menu.delete"),
				onClick: async () => {
					await deleteConfig(config.id);
					await loadStore();
				},
			},
		],
	});
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

<style></style>
