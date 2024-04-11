<template>
	<div class="grid sm:grid-rows-[2rem_minmax(0,_1fr)] md:grid-rows-[3rem_minmax(0,_1fr)] overflow-auto">
		<div class="flex flex-row justify-start items-center px-2">
			<EditGroupEnv operate="new" :configId="props.configId" @callback="loadStore(props.configId)">
				<Button variant="outline">
					<PlusSquare class="mr-2" />
					{{ t("envGroup.new") }}
				</Button>
			</EditGroupEnv>
		</div>
		<div class="sm:mt-1 md:mt-2 overflow-auto">
			<GroupEnvView v-for="group in groupEnvsState" :key="group.id" :data="group" @callback="loadStore(group.configId)"
				@remove="removeGroupEnv(group.configId, group.id)" @removeEnv="deleteStoreEnv" />
		</div>
	</div>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { deleteEnv, deleteGroupEnv, getGroupEnvs } from "@/store";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { EditGroupEnv } from "@/components/envs";
import { PlusSquare } from "lucide-vue-next";
import type { GroupEnv } from "@/store/type";
import { GroupEnv as GroupEnvView } from "@/components/envs";

const props = defineProps({
	configId: {
		type: String,
		required: true,
	},
});

console.log("props[group-env-index]:", props);

const { t } = useI18n();

const groupEnvsState = ref<GroupEnv[]>([]);

const removeGroupEnv = async (configId: string, groupEnvId: string) => {
	await deleteGroupEnv(configId, groupEnvId);
	await loadStore(configId);
};

const deleteStoreEnv = async (
	configId: string | undefined,
	groupId: string | undefined,
	envKey: string | undefined,
) => {
	if (configId && groupId && envKey) {
		await deleteEnv(configId, groupId, envKey);
		await loadStore(configId);
	}
};

const loadStore = async (configId: string | undefined) => {
	if (configId) {
		const storeGroupEnvs = await getGroupEnvs(configId);
		groupEnvsState.value = storeGroupEnvs || [];
	}
};

await loadStore(props.configId);

watch(props, async (newValue, _oldValue) => {
	await loadStore(newValue.configId);
});
</script>
