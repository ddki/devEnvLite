<template>
	<div class="grid grid-flow-col justify-between pl-8 pr-2 hover:bg-secondary rounded-md">
		<div class="grid grid-flow-col gap-2 justify-start items-center">
			<TerminalSquare class="h-4 w-4" />
			<div class="grid grid-flow-row h-full w-full justify-center">
				<div class="grid grid-flow-col gap-2 w-full justify-start items-center">
					<div class="grid grid-flow-col gap-1 items-center">
						<span>{{ props.data.key }}</span>
					</div>
					<span class="text-ellipsis text-nowrap overflow-hidden text-muted-foreground text-xs">
						{{ props.data.description }}
					</span>
				</div>
				<span class="text-ellipsis text-nowrap overflow-hidden text-muted-foreground">
					{{ props.data.value }}
				</span>
			</div>
		</div>
		<div class="grid grid-flow-col items-center">
			<EditEnvironmentVariable operate="edit" :groupId="props.groupId" :id="props.data.id">
				<Button variant="ghost" size="icon">
					<Pencil class="h-4 w-4" />
				</Button>
			</EditEnvironmentVariable>
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
						<span>{{ `${t("operate.copy")}${t("env.key")}-${t("env.value")}` }}</span>
					</DropdownMenuItem>
					<DropdownMenuItem @click="dropdownMenuCopyKey(props.data)">
						<Copy class="mr-2 h-4 w-4" />
						<span>{{ `${t("operate.copy")}${t("env.key")}` }}</span>
					</DropdownMenuItem>
					<DropdownMenuItem @click="dropdownMenuCopyValue(props.data)">
						<Copy class="mr-2 h-4 w-4" />
						<span>{{ `${t("operate.copy")}${t("env.value")}` }}</span>
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
import type { EnvironmentVariable, Res } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { Copy, Pencil, TerminalSquare, Trash2 } from "lucide-vue-next";
import { inject } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";
import { EditEnvironmentVariable } from ".";

interface Props {
	groupId: string;
	data: EnvironmentVariable;
}
const props = defineProps<Props>();

const { t } = useI18n();

const reloadVariableGroupList: () => Promise<void> =
	inject("reloadVariableGroupList") || (async () => {});

// 删除
const dropdownMenuDelete = async (data: EnvironmentVariable) => {
	const title = `${t("operate.delete")}${t("env.text")}`;
	await invoke<Res<void>>("delete_environment_variable", { groupId: props.groupId, id: data.id })
		.then(async (res) => {
			if (res.code === "200") {
				await reloadVariableGroupList();
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

// 复制键值
const dropdownMenuCopyKeyValue = async (data: EnvironmentVariable) => {
	const text = `${data.key}=${data.value}`;
	await writeText(text);
};

// 复制键
const dropdownMenuCopyKey = async (data: EnvironmentVariable) => {
	await writeText(data.key);
};

// 复制值
const dropdownMenuCopyValue = async (data: EnvironmentVariable) => {
	await writeText(data.value);
};
</script>
