<template>
	<div class="grid sm:grid-rows-[2rem_minmax(0,_1fr)] md:grid-rows-[3rem_minmax(0,_1fr)] overflow-auto">
		<div class="flex flex-row flex-2 justify-start items-center">
			<el-button type="primary" @click="importConfig">{{ t('config.import-config') }}</el-button>
			<el-button type="primary" @click="newConfig">{{ t('config.new-config') }}</el-button>
		</div>
		<div class="sm:mt-1 md:mt-2 overflow-auto">
			<ul class="menu bg-base-200 w-full rounded-box">
				<div v-for="item in configs" :key="item?.id" @click="onClickConfig(item)"
					@contextmenu="onContextMenu($event, item)">
					<li
						:class="`text-ellipsis text-nowrap overflow-hidden py-1 cursor-pointer hover:bg-blue-200 flex flex-row justify-start items-center ${item.activeClass}`">
						<el-icon v-if="item.isActive" class="mr-1">
							<CircleCheck class="text-red-500" />
						</el-icon>
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
import { CircleCheck } from "@element-plus/icons-vue";
import ContextMenu from "@imengyu/vue3-context-menu";
import { invoke } from "@tauri-apps/api/core";
import { ElNotification } from "element-plus";
import { nextTick, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { deleteConfig, getConfigs, setActiveConfigId } from "../../store/config";
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

const importConfig = async () => { };

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
	ElNotification({
		title: t("config.text"),
		message: `${config.name}`,
		position: "bottom-right",
		type: "info",
	});
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
					await deleteConfig(config);
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
.active {
	padding: .5rem;
	color: var(--el-color-white);
	background-color: var(--el-color-primary);
}
</style>
