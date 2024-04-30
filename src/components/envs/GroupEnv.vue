<template>
	<div class="grid grid-flow-col grid-cols-1 gap-2 justify-between items-center px-2 hover:bg-secondary rounded-md">
		<div class="grid grid-flow-col gap-2 justify-start items-center" @click="showItems = showItems ? false : true">
			<PanelBottomClose class="h-4 w-4" v-if="showItems === true" />
			<PanelBottomOpen class="h-4 w-4" v-else />
			<div class="grid grid-flow-col gap-2 w-full justify-start items-center">
				<span>{{ props.data.name }} ({{ props.data.envs?.length }})</span>
				<div class="grid grid-flow-col gap-2">
					<div class="grid grid-flow-col gap-1 items-center">
						<CheckCircle class="w-4 h-4" />
						<span>{{ props.data.envAppliedCount || 0 }}</span>
					</div>
					<div class="grid grid-flow-col gap-1 items-center text-destructive">
						<AlertCircle class="w-4 h-4" />
						<span>{{ props.data.envNotAppliedCount || 0 }}</span>
					</div>
				</div>
				<span class="text-ellipsis text-nowrap overflow-hidden text-muted-foreground text-xs">
					{{ props.data.note }}
				</span>
			</div>
		</div>
		<div class="grid grid-flow-col items-center">
			<EditItemEnv operate="new" :configId="props.data.configId" :groupId="props.data.id"
				@callback="editItemEnvCallback">
				<Button variant="ghost" size="icon">
					<PlusSquare class="mr-2 h-4 w-4" />
				</Button>
			</EditItemEnv>
			<EditGroupEnv operate="edit" :configId="props.data.configId" :id="props.data.id" @callback="emit('callback')">
				<Button variant="ghost" size="icon">
					<Pencil class="h-4 w-4" />
				</Button>
			</EditGroupEnv>
			<Button variant="ghost" size="icon" @click="dropdownMenuApply(props.data)">
				<CircleCheck class="h-4 w-4" />
			</Button>
			<Button variant="ghost" size="icon" @click="dropdownMenuDelete(props.data)">
				<Trash2 class="h-4 w-4 text-destructive" />
			</Button>
		</div>
	</div>
	<div class="grid grid-flow-row items-center gap-1" v-if="showItems">
		<ItemEnv v-for="env in props.data.envs" :configId="props.data.configId" :data="env" @callback="emit('callback')"
			@remove="removeEnv(env.key)"></ItemEnv>
	</div>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import type { GroupEnv } from "@/store/type";
import { invoke } from "@tauri-apps/api/core";
import {
	AlertCircle,
	CheckCircle,
	CircleCheck,
	PanelBottomClose,
	PanelBottomOpen,
	Pencil,
	PlusSquare,
	Trash2,
} from "lucide-vue-next";
import { defineEmits, defineProps, ref } from "vue";
import { useI18n } from "vue-i18n";
import { EditGroupEnv, EditItemEnv } from ".";
import { ItemEnv } from ".";
import { useToast } from "@/components/ui/toast";

interface Props {
	data: GroupEnv;
}
const props = defineProps<Props>();
const emit = defineEmits<{
	(e: "callback"): void;
	(e: "remove", configId: string, groupId: string): void;
	(e: "removeEnv", configId: string, groupId: string, envKey: string): void;
}>();

const { t } = useI18n();
const { toast } = useToast();

const showItems = ref(false);

const editItemEnvCallback = () => {
	showItems.value = true;
	emit("callback");
};
const removeEnv = (envKey: string) => {
	emit("removeEnv", props.data.configId, props.data.id, envKey);
};

// 应用
const dropdownMenuApply = async (data: GroupEnv) => {
	await invoke("group_env_apply", { configId: data.configId, groupId: data.id })
		.then(() => {
			toast({
				title: `${t("operate.apply")} ${t("envGroup.text")}`,
				description: t("message.success"),
			});
		})
		.catch((e) => {
			toast({
				title: `${t("operate.apply")} ${t("envGroup.text")}`,
				description: `${t("message.error")}: ${e.message}`,
				variant: "destructive",
			});
			console.log("dropdownMenuApply error: ", e);
		});
};

// 删除
const dropdownMenuDelete = (data: GroupEnv) => {
	emit("remove", data.configId, data.id);
};
</script>
