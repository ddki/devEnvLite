<template>
	<div class="grid sm:grid-rows-[2rem_minmax(0,_1fr)] md:grid-rows-[3rem_minmax(0,_1fr)] overflow-auto">
		<div class="flex flex-row flex-2 justify-start items-center">
			<el-button type="primary" @click="importConfig">{{ t('config.import-config') }}</el-button>
			<el-button type="primary" @click="newConfig">{{ t('config.new-config') }}</el-button>
		</div>
		<div class="sm:mt-1 md:mt-2 overflow-auto">
			<ul class="menu bg-base-200 w-full rounded-box">
				<li v-for="item in configs" :key="item?.id"><a :class="item.activeClass" @click="editConfig(item)">{{ item?.name
						}}</a></li>
			</ul>
		</div>
	</div>
	<EditConfigModal :id="configId" :title="editConfigModalTitle" v-model:visible="editConfigModalVisible"
		@postClose="postCloseEditConfigModal" />
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { nextTick, ref } from "vue";
import { useI18n } from "vue-i18n";
import { getConfigs } from "../../store/config";
import EditConfigModal from "./EditConfigModal.vue";

interface ConfigData extends Config {
	activeClass?: string;
}
const { t } = useI18n();

const configs = ref<ConfigData[]>([]);
const configId = ref("");
const editConfigModalVisible = ref(false);
const editConfigModalTitle = ref(t("config.new-config"));

const loadStore = async () => {
	const configIds = (await invoke("get_config_ids")) as string[];
	const storeConfigs = (await getConfigs(configIds)).filter((item) => item.id && item.name).sort();
	configs.value = storeConfigs;
};

await loadStore();

console.log(configs);
const importConfig = async () => {};

const newConfig = () => {
	nextTick(() => {
		configId.value = "";
		editConfigModalTitle.value = t("config.new-config");
		editConfigModalVisible.value = true;
	});
};

const editConfig = (config: ConfigData) => {
	console.log("edit config: ", config);
	resetConfigsActiveClass();
	config.activeClass = "active";
	configId.value = config.id;
	editConfigModalTitle.value = t("config.edit-config");
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
</script>
