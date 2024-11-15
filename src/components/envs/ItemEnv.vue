<template>
	<div class="grid grid-flow-col justify-between pl-8 pr-2 hover:bg-secondary rounded-md">
		<div class="grid grid-flow-col gap-2 justify-start items-center">
			<TerminalSquare class="h-4 w-4" />
			<div class="grid grid-flow-row h-full w-full justify-center">
				<div class="grid grid-flow-col gap-2 w-full justify-start items-center">
					<div class="grid grid-flow-col gap-1 items-center">
						<CheckCircle class="h-4 w-4" v-if="props.data.isApplied" />
						<AlertCircle class="h-4 w-4 text-destructive" v-else />
						<span>{{ props.data.key }}</span>
					</div>
					<span class="text-ellipsis text-nowrap overflow-hidden text-muted-foreground text-xs">
						{{ props.data.note }}
					</span>
				</div>
				<span class="text-ellipsis text-nowrap overflow-hidden text-muted-foreground">
					{{ props.data.value }}
				</span>
				<span class="text-ellipsis text-nowrap overflow-hidden text-muted-foreground" v-if="!props.data.isSame">
					{{ props.data.currentValue }}
				</span>
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
				<CircleCheck class="h-4 w-4" />
			</Button>
			<Button variant="ghost" size="icon" @click="dropdownMenuDelete(props.data)">
				<Trash2 class="h-4 w-4 text-destructive" />
			</Button>
			<DropdownMenu>
				<DropdownMenuTrigger>
					<Button variant="ghost" size="icon">
						<Copy class="h-4 w-4" />
					</Button>
				</DropdownMenuTrigger>
				<DropdownMenuContent>
					<DropdownMenuItem @click="dropdownMenuCopyKeyValue(props.data)">
						<Copy class="mr-2 h-4 w-4" />
						<span>{{ t("operate.copy", { name: `${t("env.key")}-${t("env.value")}` }) }}</span>
					</DropdownMenuItem>
					<DropdownMenuItem @click="dropdownMenuCopyKey(props.data)">
						<Copy class="mr-2 h-4 w-4" />
						<span>{{ t("operate.copy", { name: t("env.key") }) }}</span>
					</DropdownMenuItem>
					<DropdownMenuItem @click="dropdownMenuCopyValue(props.data)">
						<Copy class="mr-2 h-4 w-4" />
						<span>{{ t("operate.copy", { name: t("env.value") }) }}</span>
					</DropdownMenuItem>
				</DropdownMenuContent>
			</DropdownMenu>
		</div>
	</div>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import type { Env } from "@/store/type";
import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import {
	AlertCircle,
	CheckCircle,
	Copy,
	CircleCheck,
	Pencil,
	TerminalSquare,
	Trash2,
} from "lucide-vue-next";
import { useI18n } from "vue-i18n";
import { EditItemEnv } from ".";
import { useToast } from "@/components/ui/toast";

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
const { toast } = useToast();

// 应用
const dropdownMenuApply = async (data: Env) => {
	// todo
	await invoke("env_apply", { configId: props.configId, groupId: data.groupId, envKey: data.key, envValue: data.value })
		.then(() => {
			toast({
				title: `${t("operate.apply")} ${t("env.text")}`,
				description: t("message.success"),
			});
			emit('callback');
		})
		.catch((e) => {
			toast({
				title: `${t("operate.apply")} ${t("env.text")}`,
				description: `${t("message.error")}: ${e.message}`,
				variant: "destructive",
			});
			console.log("dropdownMenuApply error: ", e);
		});
};

// 删除
const dropdownMenuDelete = (data: Env) => {
	emit("remove", data.key);
};

// 复制键值
const dropdownMenuCopyKeyValue = async (data: Env) => {
	const text = `${data.key}=${data.value}`;
	await writeText(text);
};

// 复制键
const dropdownMenuCopyKey = async (data: Env) => {
	await writeText(data.key);
};

// 复制值
const dropdownMenuCopyValue = async (data: Env) => {
	await writeText(data.value);
};
</script>
