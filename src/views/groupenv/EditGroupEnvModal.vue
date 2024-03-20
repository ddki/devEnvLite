<template>
	<el-dialog v-model="props.visible" :title="props.title" @close="closeDialog" width="32rem" v-if="props.visible">
		<el-form ref="formRef" :model="form" label-position="right" label-width="auto">
			<el-form-item prop="id" :label="t('envGroup.id')">
				<el-input v-model="form.id" clearable :placeholder="t('envGroup.id')" />
			</el-form-item>
			<el-form-item prop="name" :label="t('envGroup.name')">
				<el-input v-model="form.name" clearable :placeholder="t('envGroup.name')" />
			</el-form-item>
			<el-form-item prop="note" :label="t('envGroup.note')">
				<el-input v-model="form.note" clearable :placeholder="t('envGroup.note')" />
			</el-form-item>
			<el-form-item prop="sort" :label="t('envGroup.sort')">
				<el-input v-model="form.sort" clearable :placeholder="t('envGroup.sort')" />
			</el-form-item>
		</el-form>
		<template #footer>
			<div class="dialog-footer">
				<el-button type="primary" @click="onSave">
					{{ t('save') }}
				</el-button>
			</div>
		</template>
	</el-dialog>
</template>

<script setup lang="ts">
import { ElNotification, type FormInstance } from "element-plus";
import { v4 as uuidv4 } from "uuid";
import { reactive, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { getConfig, getConfigNames, saveConfig } from "../../store/config";

const { t } = useI18n();

const emits = defineEmits(["update:visible", "postClose"]);
const props = defineProps({
	id: String,
	maxSort: {
		type: Number,
		default: 0,
	},
	title: String,
	visible: Boolean,
});

const formRef = ref<FormInstance>();
const form = reactive({
	id: "",
	name: "",
	note: "",
	sort: 0,
});

const onSave = async () => {
	if (!form.name) {
		ElNotification({
			title: props.title,
			message: t("envGroup.error.nameNotEmpty"),
			position: "bottom-right",
			type: "error",
		});
		return;
	}
	const configNames = await getConfigNames();
	console.log("configNames = ", configNames);
	if (configNames?.includes(form.name)) {
		ElNotification({
			title: props.title,
			message: t("envGroup.error.nameExists"),
			position: "bottom-right",
			type: "error",
		});
		return;
	}
	const save = await saveConfig(form);
	if (save) {
		ElNotification({
			title: props.title,
			message: t("save") + t("success"),
			position: "bottom-right",
			type: "success",
		});
		emits("update:visible", false);
		emits("postClose");
	} else {
		ElNotification({
			title: props.title,
			message: t("save") + t("failure"),
			position: "bottom-right",
			type: "error",
		});
	}
};

const closeDialog = () => {
	emits("update:visible", false);
	emits("postClose");
};

watch(props, async (newValue, _oldValue) => {
	if (newValue.id) {
		const storeConfig = await getConfig(newValue.id);
		form.id = storeConfig.id;
		form.name = storeConfig.name;
		form.note = storeConfig.note as string;
		form.sort = storeConfig.sort;
	} else {
		form.id = uuidv4();
		form.name = "";
		form.note = "";
		form.sort = newValue.maxSort + 1;
	}
});
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
