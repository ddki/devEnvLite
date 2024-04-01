<template>
	<div class="grid sm:grid-rows-[2rem_minmax(0,_1fr)] md:grid-rows-[3rem_minmax(0,_1fr)] overflow-auto">
		<div class="flex flex-row flex-2 justify-start items-center gap-2">
			<Button @click="importConfig">{{ t('config.import-config') }}</Button>
			<Button @click="newConfig">{{ t('config.new-config') }}</Button>
		</div>
		<div class="sm:mt-1 md:mt-2 overflow-auto">
			<span class="text-slate-500" v-if="(!configs || configs.length < 1)">{{ t("config.emptyText") }}</span>
			<ul class="menu bg-base-200 w-full rounded-box">
				<div v-for="item in configs" :key="item?.id" @click="onClickConfig(item)"
					@contextmenu="onContextMenu($event, item)">
					<li
						:class="`text-ellipsis text-nowrap overflow-hidden py-1 cursor-pointer hover:bg-blue-200 flex flex-row justify-start items-center gap-2 ${item.activeClass}`">
						<CircleCheckBig v-if="item.isActive" />
						<span>{{ item?.name }}</span>
					</li>
				</div>
			</ul>
		</div>
	</div>
	<EditConfigModal :id="configId" :title="editConfigModalTitle" :operate="editConfigModalOperate"
		v-model:visible="editConfigModalVisible" @postClose="postCloseEditConfigModal" />
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { deleteConfig, getConfigs, setActiveConfigId } from "@/store/config";
import ContextMenu from "@imengyu/vue3-context-menu";
import { invoke } from "@tauri-apps/api/core";
import { CircleCheckBig } from "lucide-vue-next";
import { nextTick, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import EditConfigModal from "./EditConfigModal.vue";

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
const editConfigModalVisible = ref(false);
const editConfigModalTitle = ref(t("config.new-config"));
const editConfigModalOperate = ref("new");

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

const importConfig = async () => {};

const newConfig = () => {
	nextTick(() => {
		configId.value = "";
		editConfigModalTitle.value = t("config.new-config");
		editConfigModalOperate.value = "new";
		editConfigModalVisible.value = true;
	});
};

const editConfig = (config: ConfigData) => {
	console.log("edit config: ", config);
	resetConfigsActiveClass();
	config.activeClass = "active";
	configId.value = config.id;
	editConfigModalTitle.value = t("config.edit-config");
	editConfigModalOperate.value = "edit";
	editConfigModalVisible.value = true;
};

const postCloseEditConfigModal = async () => {
	await loadStore();
	configId.value = "";
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

<style>

</style>
