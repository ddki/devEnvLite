<template>
	<div class="flex flex-row" v-if="errorMessage != null">
		<div class="alert alert-error">
			{{ errorMessage }}
		</div>
	</div>
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

</template>

<script setup lang="ts">
import { v4 as uuidv4 } from "uuid";
import { getCurrentInstance, reactive, ref } from "vue";
import { useI18n } from "vue-i18n";
import { getConfig, getConfigNames, saveConfig } from "../../store/config";

const { t } = useI18n();
const global = getCurrentInstance()?.appContext.config.globalProperties;

const emits = defineEmits(["callBack"]);
const props = defineProps({
	id: String,
});

const initConfig = {
	id: uuidv4(),
	name: "",
	note: "",
	sort: null,
};

const storeConfig = (props.id ? await getConfig(props.id) : initConfig) || initConfig;

const saveData = reactive<Config>({
	id: storeConfig?.id,
	name: storeConfig.name,
	note: storeConfig.note,
	sort: storeConfig.sort || 0,
});

const errorMessage = ref();

const onSave = async () => {
	if (!saveData.name) {
		global?.$toast.warning(`${t("config.name")}不能为空`, ["center", "middle"]);
		errorMessage.value = `${t("config.name")}不能为空`
		return;
	}
	const configNames = await getConfigNames();
	console.log("configNames = ", configNames);
	if (configNames?.includes(saveData.name)) {
		global?.$toast.warning(`${t("config.name")}已经存在`);
		return;
	}
	const save = await saveConfig(saveData);
	console.log(save);
	if (save) {
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
