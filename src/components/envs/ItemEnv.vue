<template>
	<div class="grid grid-flow-col justify-between px-2 hover:bg-secondary rounded-md">
		<div class="grid grid-flow-col gap-2 justify-start items-center">
			<TerminalSquare class="h-4 w-4" />
			<div class="grid grid-flow-col gap-2 w-full justify-start items-center">
				<span>{{ props.data.key }}</span>
				<span class="text-ellipsis text-nowrap overflow-hidden text-muted-foreground text-sm">{{ props.data.note }}</span>
			</div>
		</div>
		<div class="grid grid-flow-col items-center">
			<EditItemEnv operate="edit" :configId="props.configId" :groupId="props.data.groupId" :env-key="props.data.key"
				@callback="emit('callback')">
				<Button variant="ghost" size="icon">
					<Pencil class="h-4 w-4" />
				</Button>
			</EditItemEnv>
			<Button variant="ghost" size="icon" @click="dropdownMenuApply(props.data)">
				<Laugh class="h-4 w-4" />
			</Button>
			<Button variant="ghost" size="icon" @click="dropdownMenuDelete(props.data)">
				<Trash2 class="h-4 w-4 text-destructive" />
			</Button>
		</div>
	</div>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import type { Env } from "@/store/type";
import { invoke } from "@tauri-apps/api/core";
import { Laugh, Pencil, TerminalSquare, Trash2 } from "lucide-vue-next";
import { defineEmits, defineProps } from "vue";
import { useI18n } from "vue-i18n";
import { EditItemEnv } from ".";

interface Props {
	configId: string;
	data: Env;
}
const props = defineProps<Props>();
const emit = defineEmits<{
	(e: "callback"): void;
	(e: "remove", envKey: string): void;
}>();

const { t } = useI18n();

// 应用
const dropdownMenuApply = async (data: Env) => {
	// todo
	await invoke("env_apply");
};

// 删除
const dropdownMenuDelete = (data: Env) => {
	emit("remove", data.key);
};
</script>
