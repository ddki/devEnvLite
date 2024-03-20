<template>
	<div class="grid sm:grid-rows-[2rem_minmax(0,_1fr)] md:grid-rows-[3rem_minmax(0,_1fr)] overflow-auto">
		<div class="flex flex-row flex-2 justify-start items-center">
			<el-button type="primary" @click="newGroupEnv">{{ t('envGroup.new') }}</el-button>
			<EditGroupEnvModal />
		</div>
		<div class="sm:mt-1 md:mt-2 overflow-auto">
			<p class="h-60">配置1</p>
			<p class="h-60">配置1</p>
			<p class="h-60">配置1</p>
			<p class="h-60">配置1</p>
			<p class="h-60">配置1</p>
		</div>
	</div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { getConfig } from "../../store/config";
import { reactive, ref, watch } from "vue";
import { ElNotification } from "element-plus";
import EditGroupEnvModal from "./EditGroupEnvModal.vue";


const props = defineProps({
	configId: String
});

const { t } = useI18n();
const storeConfig = props.configId ? await getConfig(props.configId) : null;
const editGroupEnvModal = ref(false);


let groupEnvsState = reactive<GroupEnv[]>([]);

const newGroupEnv = () => {
	if (storeConfig) {

	} else {
		ElNotification({
			title: t("envGroup.new"),
			message: t("envGroup.error.selectConfig"),
			position: "bottom-right",
			type: "warning"
		})
	}
};

watch(props, async (newValue, oldValue) => {
	if (props.configId) {
		const storeConfig = await getConfig(props.configId);
		groupEnvsState = storeConfig.groupEnvs || [];
	}
})
</script>
