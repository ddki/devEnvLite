<template>
	<div class="grid sm:grid-rows-[2rem_minmax(0,_1fr)] md:grid-rows-[3rem_minmax(0,_1fr)] overflow-auto">
		<div class="flex flex-row flex-2 justify-start items-center">
			<el-button type="primary" @click="newGroupEnv">{{ t('envGroup.new') }}</el-button>
			<EditGroupEnvModal v-model:visible="editGroupEnvModalVisible" :title="editGroupEnvModalTitle"
				:operate="editGroupEnvModalOperate" :id="editGroupEnvId" :configId="props.configId"
				@postClose="postCloseEditConfigModal" />
		</div>
		<div class="sm:mt-1 md:mt-2 overflow-auto">
			<el-tree ref="groupEnvsTreeRef" :data="groupEnvsState" :props="groupEnvsTreeProps" node-key="id"
				@node-click="onClickNode" @node-contextmenu="nodeContextMenu">
				<template #default="{ node, data }">
					<div class="flex flex-row justify-between w-full">
						<span>{{ node.label || data.key }}</span>
						<span v-if="isGroupNode(data)">
							分组
						</span>
						<span v-if="!isGroupNode(data)">
							环境变量
						</span>
					</div>
				</template>
			</el-tree>
		</div>
	</div>
	<EditEnvModal v-model:visible="editEnvModalVisible" :title="editEnvModalTitle" :operate="editEnvModalOperate"
		:envKey="envKey" :groupId="editGroupEnvId" :configId="props.configId" @postClose="postCloseEditEnvModal" />
</template>

<script setup lang="ts">
import ContextMenu from "@imengyu/vue3-context-menu";
import { ElNotification, type ElTree } from "element-plus";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { deleteEnv, getGroupEnvs } from "../../store/config";
import { getConfig, saveConfig } from "../../store/config";
import EditEnvModal from "./EditEnvModal.vue";
import EditGroupEnvModal from "./EditGroupEnvModal.vue";

const props = defineProps({
	configId: {
		type: String,
		required: true,
	},
});

const { t } = useI18n();
const groupEnvsTreeRef = ref<InstanceType<typeof ElTree>>();

const editGroupEnvModalVisible = ref(false);
const editGroupEnvModalTitle = ref("");
const editGroupEnvModalOperate = ref("new");
const editGroupEnvId = ref("");

const editEnvModalVisible = ref(false);
const editEnvModalTitle = ref("");
const editEnvModalOperate = ref("new");
const envKey = ref("");

const groupEnvsState = ref<GroupEnv[]>([]);
const groupEnvsTreeProps = {
	children: "envs",
	label: "name",
};

const newGroupEnv = () => {
	if (!props.configId) {
		ElNotification({
			title: t("envGroup.new"),
			message: t("envGroup.error.selectConfig"),
			position: "bottom-right",
			type: "warning",
		});
		return;
	}
	editGroupEnvModalVisible.value = true;
	editGroupEnvModalTitle.value = t("envGroup.new");
	editGroupEnvModalOperate.value = "new";
};

const editGroupEnv = (groupEnvId: string) => {
	editGroupEnvModalVisible.value = true;
	editGroupEnvModalTitle.value = t("envGroup.edit");
	editGroupEnvModalOperate.value = "edit";
	editGroupEnvId.value = groupEnvId;
	ElNotification({
		title: t("envGroup.edit"),
		message: t("envGroup.error.selectConfig"),
		position: "bottom-right",
		type: "warning",
	});
};

// biome-ignore lint/suspicious/noExplicitAny: <explanation>
const onClickNode = (obj: any, node: any, treeNode: any, evnet: PointerEvent) => {
	// console.log(obj, node, treeNode, evnet);
	ElNotification({
		title: t("envGroup.text"),
		message: `选中了${obj.name || obj.key}`,
		position: "bottom-right",
		type: "success",
	});
};

// biome-ignore lint/suspicious/noExplicitAny: <explanation>
const nodeContextMenu = (event: MouseEvent, obj: any) => {
	console.log(event, obj);
	event.preventDefault();
	if (isGroupNode(obj)) {
		ContextMenu.showContextMenu({
			x: event.x,
			y: event.y,
			items: [
				{
					label: t("envGroup.context-menu.addEnv"),
					onClick: () => {
						editEnvModalVisible.value = true;
						editEnvModalTitle.value = t("env.new");
						editEnvModalOperate.value = "new";
						editGroupEnvId.value = obj.id;
					},
				},
				{
					label: t("envGroup.context-menu.check"),
					onClick: () => {
						// todo 调用rust
					},
				},
				{
					label: t("envGroup.context-menu.apply"),
					onClick: () => {
						// todo 调用rust
					},
				},
				{
					label: t("envGroup.context-menu.modify"),
					onClick: () => {
						editGroupEnv(obj.id);
					},
				},
				{
					label: t("envGroup.context-menu.delete"),
					onClick: async () => {
						removeGroupEnvs(obj.id);
						await saveStore(props.configId);
						await loadStore(props.configId);
					},
				},
			],
		});
	} else {
		ContextMenu.showContextMenu({
			x: event.x,
			y: event.y,
			items: [
				{
					label: t("envGroup.context-menu.apply"),
					onClick: () => {
						// todo 调用rust
					},
				},
				{
					label: t("envGroup.context-menu.modify"),
					onClick: () => {
						console.log("obj == ", obj);
						editEnvModalVisible.value = true;
						editEnvModalTitle.value = t("env.edit");
						editEnvModalOperate.value = "edit";
						editGroupEnvId.value = obj.groupId;
						envKey.value = obj.key;
					},
				},
				{
					label: t("envGroup.context-menu.delete"),
					onClick: async () => {
						await deleteStoreEnv(props.configId, obj.groupId, obj.key);
						await loadStore(props.configId);
					},
				},
			],
		});
	}
};

// biome-ignore lint/suspicious/noExplicitAny: <explanation>
const isGroupNode = (data: any): boolean => {
	if (data.name) {
		return true;
	}
	return false;
};

const removeGroupEnvs = (groupEnvId: string) => {
	if (groupEnvsState.value) {
		const newGroupEnvs = groupEnvsState.value.filter((item) => item.id !== groupEnvId);
		groupEnvsState.value = newGroupEnvs;
	}
};

const saveStore = async (configId: string | undefined) => {
	if (configId) {
		const storeConfig = await getConfig(configId);
		storeConfig.groupEnvs = groupEnvsState.value;
		await saveConfig(storeConfig);
	}
};

const deleteStoreEnv = async (
	configId: string | undefined,
	groupId: string | undefined,
	envKey: string | undefined,
) => {
	if (configId && groupId && envKey) {
		await deleteEnv(configId, groupId, envKey);
	}
};

const loadStore = async (configId: string | undefined) => {
	if (configId) {
		const storeGroupEnvs = await getGroupEnvs(configId);
		groupEnvsState.value = storeGroupEnvs || [];
	}
};

const postCloseEditConfigModal = async () => {
	await loadStore(props.configId);
};

const postCloseEditEnvModal = async () => {
	await loadStore(props.configId);
};

await loadStore(props.configId);

watch(props, async (newValue, _oldValue) => {
	await loadStore(newValue.configId);
});
</script>
