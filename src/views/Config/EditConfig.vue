<template>
	<dialog id="config_modal" ref="configModal" class="modal modal-bottom sm:modal-middle">
		<div class="modal-box">
			<form method="dialog">
				<button class="btn btn-sm sm:btn-sm md:btn-md btn-circle btn-ghost absolute right-2 top-2">✕</button>
			</form>
			<h3 class="font-bold text-lg">{{ props.title }}</h3>
			<p class="py-4">
			<div class="item">
				<span>
					{{ t('config.id') }}
				</span>
				<input type="text" readonly class="input input-bordered w-full max-w-xs" v-model="saveData.id"
					:placeholder="t('config.id')" />
			</div>
			<div class="item">
				<span>
					{{ t('config.name') }}
				</span>
				<input type="text" autofocus class="input input-bordered input-primary w-full max-w-xs" v-model="saveData.name"
					:placeholder="t('config.name')" />
			</div>
			<div class="item">
				<span>
					{{ t('config.sort') }}
				</span>
				<input type="number" min="0" class="input input-bordered input-primary w-full max-w-xs" v-model="saveData.sort"
					:placeholder="t('config.sort')" />
			</div>
			<div class="item">
				<span>
					{{ t('config.note') }}
				</span>
				<input type="text" class="input input-bordered input-primary w-full max-w-xs" v-model="saveData.note"
					:placeholder="t('config.note')" />
			</div>
			<div class="flex flex-row justify-end mt-8">
				<button type="button" class="btn btn-primary btn-sm sm:btn-sm md:btn-md" @click="onSave">
					{{ t('save') }}
				</button>
			</div>
			</p>
		</div>
	</dialog>

</template>

<script setup lang="ts">
import { v4 as uuidv4 } from "uuid";
import { getCurrentInstance, onUnmounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { getConfig, getConfigNames, saveConfig } from "../../store/config";

const { t } = useI18n();
const global = getCurrentInstance()?.appContext.config.globalProperties;

const emits = defineEmits(["callBack"]);
const props = defineProps({
	id: String,
	maxSort: {
		type: Number,
		default: 0
	},
	title: String
});

console.log('props = ', props)
const configModal = ref(null)

onUnmounted(() => {
	const modal1 = document.getElementById("config_modal") as HTMLDialogElement;
	modal1.showModal();
	const modal = configModal.value as unknown as HTMLDialogElement;
	modal.showModal();
});

const initConfig = {
	id: uuidv4(),
	name: "",
	note: "",
	sort: props.maxSort + 1,
};

const storeConfig = (props.id ? await getConfig(props.id) : initConfig) || initConfig;

const saveData = ref<Config>({
	id: storeConfig?.id,
	name: storeConfig.name,
	note: storeConfig.note,
	sort: storeConfig.sort,
});

const onSave = async () => {
	if (!saveData.value.name) {
		global?.$toast.warning(`${t("config.error.nameNotEmpty")}`);
		return;
	}
	const configNames = await getConfigNames();
	console.log("configNames = ", configNames);
	if (configNames?.includes(saveData.value.name)) {
		global?.$toast.warning(`${t("config.error.nameExists")}`);
		return;
	}
	const save = await saveConfig(saveData.value);
	console.log(save);
	if (save) {
		const modal = configModal.value as unknown as HTMLDialogElement;
		modal.close();
		emits("callBack");
	} else {
		global?.$toast.warning(t("save") + t("failure"));
	}
};
</script>

<style scoped>
.item {
	display: grid;
	grid-auto-columns: 1fr;
	grid-template-columns: 1fr 2fr;
	gap: 0rem 1rem;
	justify-content: center;
	align-content: center;
	justify-items: end;
	align-items: center;
	padding: 0.2rem 0;
}
</style>
