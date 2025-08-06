<template>
	<div class="grid grid-flow-col grid-cols-1 gap-2 justify-between items-center px-2 hover:bg-secondary rounded-md">
		<div class="grid grid-flow-col gap-2 justify-start items-center" @click="showItems = showItems ? false : true">
			<PanelBottomClose class="h-4 w-4" v-if="showItems === true" />
			<PanelBottomOpen class="h-4 w-4" v-else />
			<div class="grid grid-flow-col gap-2 w-full justify-start items-center">
				<span>{{ props.data.name }} ({{ props.data.variables?.length }})</span>
				<span class="text-ellipsis text-nowrap overflow-hidden text-muted-foreground text-xs">
					{{ props.data.description }}
				</span>
			</div>
		</div>
		<div class="grid grid-flow-col items-center">
			<EditEnvironmentVariable operate="new" :configId="props.data.configId" :groupId="props.data.id"
				@callback="editItemEnvCallback">
				<Button variant="ghost" size="icon">
					<PlusSquare class="mr-2 h-4 w-4" />
				</Button>
			</EditEnvironmentVariable>
			<EditEnvironmentGroup operate="edit" :configId="props.data.configId" :id="props.data.id" @callback="emit('callback')">
				<Button variant="ghost" size="icon">
					<Pencil class="h-4 w-4" />
				</Button>
			</EditEnvironmentGroup>
			<Button variant="ghost" size="icon" @click="dropdownMenuDelete(props.data)">
				<Trash2 class="h-4 w-4 text-destructive" />
			</Button>
		</div>
	</div>
	<div class="grid grid-flow-row items-center gap-1" v-if="showItems">
		<EnvironmentVariable v-for="env in props.data.variables" :configId="props.data.configId" :data="env" @callback="emit('callback')"
			@remove="removeEnv(env.key)"></EnvironmentVariable>
	</div>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import type { Res, VariableGroup } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { PanelBottomClose, PanelBottomOpen, Pencil, PlusSquare, Trash2 } from "lucide-vue-next";
import { inject, ref } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";
import { EditEnvironmentGroup, EditEnvironmentVariable, EnvironmentVariable } from ".";

interface Props {
	data: VariableGroup;
}
const props = defineProps<Props>();

const { t } = useI18n();

const showItems = ref(false);

// 删除环境变量组
const dropdownMenuDelete = async (data: VariableGroup) => {
	const title = `${t("operate.delete")}${t("envGroup.text")}`;
	if (!data.id) {
		toast.error(title, {
			description: `${t("envGroup.id")}${t("message.not-empty")}`,
		});
		return;
	}
	await invoke<Res<void>>("delete_variable_group", { id: data.id })
		.then(async (res) => {
			if (res.code === "200") {
				await inject("reloadVariableGroupList");
			} else {
				toast.error(title, {
					description: `${t("message.error")}: ${res.message}`,
				});
			}
		})
		.catch((e) => {
			toast.error(title, {
				description: `${t("message.error")}: ${e.message}`,
			});
		});
};
</script>
