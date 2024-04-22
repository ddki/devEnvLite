<template>
	<div class="h-full w-full grid grid-rows-[3.5rem_1fr]">
		<div class="flex flex-row justify-start items-center px-2 border-b">
			<EditGroupEnv operate="new" :configId="props.configId" @callback="loadStore(props.configId)">
				<Button variant="outline">
					<PlusSquare class="mr-2" />
					{{ t("operate.new", { name: t("envGroup.text") }) }}
				</Button>
			</EditGroupEnv>
		</div>
		<ScrollArea class="h-full w-full p-2">
			<div class="grid grid-flow-row gap-2 overflow-auto">
				<GroupEnvView v-for="group in groupEnvsState" :data="group" @callback="loadStore(group.configId)"
					@remove="removeGroupEnv" @removeEnv="deleteStoreEnv" />
			</div>
		</ScrollArea>
	</div>
</template>

<script setup lang="ts">
import { EditGroupEnv } from "@/components/envs";
import { GroupEnv as GroupEnvView } from "@/components/envs";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { deleteEnv, deleteGroupEnv, getGroupEnvs } from "@/store";
import type { GroupEnv } from "@/store/type";
import { PlusSquare } from "lucide-vue-next";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";

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

const deleteStoreEnv = async (configId: string, groupId: string, envKey: string) => {
	console.log("deleteStoreEnv: ", configId, groupId, envKey);
	if (configId && groupId && envKey) {
		await deleteEnv(configId, groupId, envKey);
		await loadStore(configId);
	}
};

const loadStore = async (configId: string | undefined) => {
	if (configId) {
		const storeGroupEnvs = await getGroupEnvs(configId);
		console.log("storeGroupEnvs : ", storeGroupEnvs);
		groupEnvsState.value = storeGroupEnvs || [];
	}
};

await loadStore(props.configId);

watch(props, async (newValue, _oldValue) => {
	await loadStore(newValue.configId);
});
</script>
