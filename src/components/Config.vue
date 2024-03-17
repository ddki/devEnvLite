<template>
	<div class="grid sm:grid-rows-[2rem_minmax(0,_1fr)] md:grid-rows-[3rem_minmax(0,_1fr)] shadow-xl">
		<div class="flex flex-row flex-2 justify-start items-center">
			<button type="button" @click="importConfig" class="btn btn-primary btn-sm sm:btn-sm md:btn-md sm:mr-2 md:mr-4">
				{{ t('config.import-config') }}
			</button>
			<button type="button" @click="newConfig" class="btn btn-primary btn-sm sm:btn-sm md:btn-md">
				{{ t('config.new-config') }}
			</button>
		</div>
		<div>
			<p v-for="item in configs" :key="item?.id">{{ item?.name }}</p>
		</div>
	</div>
	<dialog id="new_config_modal" class="modal modal-bottom sm:modal-middle">
		<div class="modal-box">
			<form method="dialog">
				<button class="btn btn-sm sm:btn-sm md:btn-md btn-circle btn-ghost absolute right-2 top-2">✕</button>
			</form>
			<h3 class="font-bold text-lg">{{ t('config.new-config') }}</h3>
			<p class="py-4">
				<Suspense>
					<template #default>
						<NewConfig @callBack="postSaveSetting" />
					</template>
				</Suspense>
			</p>
		</div>
	</dialog>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { defineAsyncComponent, reactive, watch } from "vue";
import { useI18n } from "vue-i18n";
import { getConfigs } from "../store/config";
const NewConfig = defineAsyncComponent(() => import("./Config/NewConfig.vue"));

const { t } = useI18n();

let configs = reactive<Config[]>([]);

const loadStore = async () => {
	const configIds = (await invoke("get_config_ids")) as string[];
	const storeConfigs = (await getConfigs(configIds)).filter((item) => item.id && item.name);
	configs = storeConfigs;
};

await loadStore();

console.log(configs);
const importConfig = async () => {};

const newConfig = () => {
	const modal = document.getElementById("new_config_modal") as HTMLDialogElement;
	modal.showModal();
};

const postSaveSetting = async () => {
	const modal = document.getElementById("new_config_modal") as HTMLDialogElement;
	modal.close();
	await loadStore();
};
</script>
