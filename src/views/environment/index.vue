<template>
	<div class="h-full w-full grid grid-rows-[3.5rem_1fr]">
		<div class="flex flex-row justify-between items-center px-2 border-b">
			<div class="flex flex-row gap-1 items-center">
				<span>{{ config.name }}</span>
				<span class="text-ellipsis text-nowrap overflow-hidden text-muted-foreground text-xs">
					{{ config.description }}
				</span>
			</div>
			<EditVariableGroup operate="new" :configId="config.id" @reload="loadVariableGroupList(config.id)">
				<Button variant="outline">
					<Blocks />
					{{ `${t("operate.new")}${t("envGroup.text")}` }}
				</Button>
			</EditVariableGroup>
		</div>
		<ScrollArea class="h-full w-full p-2 overflow-auto">
			<div class="grid grid-flow-row gap-2">
				<VariableGroupComponent v-for="group in variableGroupListState" :data="group" @reload="loadVariableGroupList(group.configId)" />
			</div>
		</ScrollArea>
	</div>
</template>

<script setup lang="ts">
import { EditVariableGroup, VariableGroup as VariableGroupComponent } from "@/components/envs";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import type { Res, VariableGroup } from "@/types";
import type { EnvConfig } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { Blocks } from "lucide-vue-next";
import { provide, ref, toRefs, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

interface Props {
	config: EnvConfig;
}

const props = defineProps<Props>();

const { config } = toRefs(props);

console.log("props[group-env-index]:", props);

const { t } = useI18n();

const variableGroupListState = ref<VariableGroup[]>([]);

// 根据配置ID加载环境变量组列表
const loadVariableGroupList = async (configId: string | undefined) => {
	const title = `${t("operate.query")}${t("envGroup.text")}`;
	if (!configId) {
		toast.error(title, {
			description: `${t("config.id")}${t("message.not-empty")}`,
		});
		return;
	}
	await invoke<Res<VariableGroup[]>>("list_variable_groups_with_variables", { configId })
		.then(async (res) => {
			if (res.code === "200") {
				variableGroupListState.value = res.data || [];
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

// 初始化加载环境变量组列表
await loadVariableGroupList(config.value.id);

// 监听配置ID变化，重新加载环境变量组列表
watch(
	() => config.value.id,
	async (newValue, _oldValue) => {
		await loadVariableGroupList(newValue);
	},
);

provide("configId", config.value.id);
provide("reloadVariableGroupList", loadVariableGroupList);
provide<VariableGroup[]>("variableGroupList", variableGroupListState.value);
</script>
