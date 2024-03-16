<template>
	<div class="item">
		<span>
			{{ t('config.name') }}
		</span>
		<input class="input input-bordered input-primary w-full max-w-xs" v-model="saveData.name"
			:placeholder="t('config.name')" />
	</div>
	<div class="item">
		<span>
			{{ t('config.sort') }}
		</span>
		<input class="input input-bordered input-primary w-full max-w-xs" v-model="saveData.sort"
			:placeholder="t('config.sort')" />
	</div>
	<div class="item">
		<span>
			{{ t('config.note') }}
		</span>
		<input class="input input-bordered input-primary w-full max-w-xs" v-model="saveData.note"
			:placeholder="t('config.note')" />
	</div>

	<div class="flex flex-row justify-end mt-8">
		<button type="button" class="btn btn-primary btn-sm sm:btn-sm md:btn-md" @click="onSave">
			{{ t('save') }}
		</button>
	</div>

</template>

<script setup lang="ts">
import { getCurrentInstance, reactive } from "vue";
import { useI18n } from "vue-i18n";
import { getConfig, saveConfig } from "../../store/config";

const { t } = useI18n();
const global = getCurrentInstance()?.appContext.config.globalProperties;

const emits = defineEmits(["callBack"]);
const props = defineProps({
	id: String,
});

const storeConfig = props.id
	? await getConfig(props.id)
	: {
			id: "",
			name: "",
			note: "",
			sort: 0,
	  };

const saveData = reactive({
	id: storeConfig.id,
	name: storeConfig.name,
	note: storeConfig.note,
	sort: storeConfig.sort,
});

const onSave = async () => {
	const save = await saveConfig({
		id: saveData.id,
		name: saveData.name,
		note: saveData.note,
		sort: saveData.sort,
	});
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
	gap: 0em 1rem;
	justify-content: center;
	align-content: center;
	justify-items: end;
	align-items: center;
}
</style>
