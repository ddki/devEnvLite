<template>
	<div class="grid sm:grid-rows-[2rem_minmax(0,_1fr)] md:grid-rows-[3rem_minmax(0,_1fr)] overflow-auto">
		<div class="flex flex-row flex-2 justify-start items-center">
			<el-button type="primary" @click="newGroupEnv">{{ t('envGroup.new') }}</el-button>
			<EditGroupEnvModal v-model:visible="editGroupEnvModalVisible" :title="editGroupEnvModalTitle" :id="editGroupEnvId"
				:configId="props.configId" @postClose="postCloseEditConfigModal" />
		</div>
		<div class="sm:mt-1 md:mt-2 overflow-auto">
			<p v-for="item in groupEnvsState" :key="item.id">
				{{ item.name }}
			</p>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ElNotification } from "element-plus";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { getGroupEnvs } from "../../store/config";
import EditGroupEnvModal from "./EditGroupEnvModal.vue";

const props = defineProps({
	configId: String,
});

const { t } = useI18n();
const editGroupEnvModalVisible = ref(false);
const editGroupEnvModalTitle = ref("");
const editGroupEnvId = ref("");

const groupEnvsState = ref<GroupEnv[]>([]);

const newGroupEnv = () => {
	editGroupEnvModalVisible.value = true;
	editGroupEnvModalTitle.value = t("envGroup.new");
	ElNotification({
		title: t("envGroup.new"),
		message: t("envGroup.error.selectConfig"),
		position: "bottom-right",
		type: "warning",
	});
};

const loadStore = async (configId: string | undefined) => {
	if (configId) {
		const storeGroupEnvs = await getGroupEnvs(configId);
		console.log("loadStore configId, storeconfig =", configId, storeGroupEnvs);
		groupEnvsState.value = storeGroupEnvs || [];
	}
	console.log("groupEnv loadStore env: ", groupEnvsState);
};

const postCloseEditConfigModal = async () => {
	await loadStore(props.configId);
};

await loadStore(props.configId);

watch(props, async (newValue, _oldValue) => {
	await loadStore(newValue.configId);
});
</script>
