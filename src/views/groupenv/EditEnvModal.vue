<template>
	<el-dialog v-model="props.visible" :title="props.title" @close="closeDialog" width="32rem" v-if="props.visible">
		<el-form ref="envFormRef" :model="form" label-position="right" label-width="auto">
			<el-form-item prop="groupId" :label="t('envGroup.id')">
				<el-input v-model="form.groupId" clearable :placeholder="t('envGroup.id')" readonly />
			</el-form-item>
			<el-form-item prop="key" :label="t('env.key')">
				<el-input v-model="form.key" clearable :placeholder="t('env.key')" />
			</el-form-item>
			<el-form-item prop="value" :label="t('env.value')">
				<el-input v-model="form.value" clearable :placeholder="t('env.value')" />
			</el-form-item>
			<el-form-item prop="note" :label="t('env.note')">
				<el-input v-model="form.note" clearable :placeholder="t('env.note')" />
			</el-form-item>
			<el-form-item prop="sort" :label="t('env.sort')">
				<el-input v-model="form.sort" clearable :placeholder="t('env.sort')" />
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
import { reactive, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { getEnv, getGroupEnv, saveEnvToGroup } from "../../store/config";

const { t } = useI18n();

const emits = defineEmits(["update:visible", "postClose"]);
const props = defineProps({
	envKey: String,
	configId: {
		type: String,
		required: true,
	},
	groupId: {
		type: String,
		required: true,
	},
	maxSort: {
		type: Number,
		default: 0,
	},
	title: String,
	visible: Boolean,
	operate: {
		type: String,
		required: true,
	},
});

console.log("env props = ", props);

const envFormRef = ref<FormInstance>();
const form = reactive({
	groupId: props.groupId,
	key: props.envKey || "",
	value: "",
	note: "",
	sort: 0,
});

const onSave = async () => {
	if (!props.configId || !form.groupId) {
		ElNotification({
			title: props.title,
			message: t("env.error.selectGroup"),
			position: "bottom-right",
			type: "error",
		});
		return;
	}
	if (!form.key) {
		ElNotification({
			title: props.title,
			message: t("env.error.keyNotEmpty"),
			position: "bottom-right",
			type: "error",
		});
		return;
	}
	if (
		props.operate === "new" &&
		(await checkGroupEnvsKeyExists(props.configId, form.groupId, form.key))
	) {
		ElNotification({
			title: props.title,
			message: t("env.error.keyExists"),
			position: "bottom-right",
			type: "error",
		});
		return;
	}
	console.log("form: ", form);
	const save = await saveEnvToGroup(props.configId, form);
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
};

const checkGroupEnvsKeyExists = async (configId: string, groupId: string, envKey: string) => {
	if (props.envKey === envKey) {
		return false;
	}
	const result = await getGroupEnv(configId, groupId).then((group) => {
		if (group?.envs) {
			return group.envs.map((env) => env.key).includes(envKey);
		}
		return false;
	});
	return result;
};

const loadStoreEnv = async (configId: string, groupId: string, envKey: string) => {
	await getEnv(configId, groupId, envKey).then((env) => {
		if (env) {
			form.groupId = env.groupId;
			form.key = env.key;
			form.value = env.value;
			form.note = env.note || "";
			form.sort = env.sort;
		}
	});
};

watch(props, async (newValue, _oldValue) => {
	if (newValue.envKey && newValue.configId && newValue.groupId) {
		await loadStoreEnv(newValue.configId, newValue.groupId, newValue.envKey);
	} else {
		form.groupId = newValue.groupId;
		form.key = "";
		form.value = "";
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
