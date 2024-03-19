<template>
	<div class="grid sm:grid-rows-[2rem_minmax(0,_1fr)] md:grid-rows-[3rem_minmax(0,_1fr)] overflow-auto">
		<div class="flex flex-row flex-2 justify-start items-center">
			<button type="button" @click="importConfig" class="btn btn-primary btn-sm sm:btn-sm md:btn-md sm:mr-2 md:mr-4">
				{{ t('config.import-config') }}
			</button>
			<button type="button" @click="newConfig" class="btn btn-primary btn-sm sm:btn-sm md:btn-md">
				{{ t('config.new-config') }}
			</button>
		</div>
		<div class="sm:mt-1 md:mt-2 overflow-auto">
			<ul class="menu bg-base-200 w-full rounded-box">
				<li v-for="item in configs" :key="item?.id"><a :class="item.activeClass" @click="editConfig(item)">{{ item?.name
						}}</a></li>
			</ul>
		</div>
	</div>
	<Suspense>
		<template #default>
			<EditConfig :id="configId" @callBack="postSaveSetting" v-if="editConfigDialog" />
		</template>
	</Suspense>
</template>

<script setup lang="ts">
interface ConfigData extends Config {
	activeClass?: string;
}
import { invoke } from "@tauri-apps/api/core";
import { defineAsyncComponent, nextTick, ref } from "vue";
import { useI18n } from "vue-i18n";
import { getConfigs } from "../store/config";
const EditConfig = defineAsyncComponent(() => import("./Config/EditConfig.vue"));

const { t } = useI18n();

const configs = ref<ConfigData[]>([]);
const configId = ref();
const editConfigDialog = ref(false);
const editConfigDialogTitle = ref(t("config.new-config"));

const loadStore = async () => {
	const configIds = (await invoke("get_config_ids")) as string[];
	const storeConfigs = (await getConfigs(configIds)).filter((item) => item.id && item.name).sort();
	configs.value = storeConfigs;
};

await loadStore();

console.log(configs);
const importConfig = async () => { };

const newConfig = () => {
	configId.value = "";
	editConfigDialogTitle.value = t("config.new-config");
	editConfigDialog.value = true;
};

const editConfig = (config: ConfigData) => {
	console.log('edit config: ', config)
	resetConfigsActiveClass();
	config.activeClass = "active";
	configId.value = config.id;
	editConfigDialogTitle.value = t("config.edit-config");
	editConfigDialog.value = true;
};

const postSaveSetting = async () => {
	editConfigDialog.value = false;
	location.reload;
	await loadStore();
};

const resetConfigsActiveClass = () => {
	configs.value = configs.value
		.map((item) => {
			item.activeClass = "";
			return item;
		});
};
</script>
