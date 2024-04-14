<template>
	<div class="grid grid-flow-col grid-cols-1 gap-2 justify-between items-center px-2 hover:bg-secondary rounded-md">
		<div class="grid grid-flow-col gap-2 justify-start items-center" @click="showItems = showItems ? false : true">
			<PanelBottomClose class="h-4 w-4" v-if="showItems === true" />
			<PanelBottomOpen class="h-4 w-4" v-else />
			<div class="grid grid-flow-col gap-2 w-full justify-start items-center">
				<span>{{ props.data.name }}</span>
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
			<DropdownMenu>
				<DropdownMenuTrigger>
					<Button variant="ghost" size="icon">
						<Ellipsis class="h-4 w-4" />
					</Button>
				</DropdownMenuTrigger>
				<DropdownMenuContent>
					<DropdownMenuItem @click="dropdownMenuCheck(props.data)">
						<SearchCheck class="mr-2 h-4 w-4" />
						<span>{{ t("envGroup.context-menu.check") }}</span>
					</DropdownMenuItem>
					<DropdownMenuItem @click="dropdownMenuApply(props.data)">
						<Laugh class="mr-2 h-4 w-4" />
						<span>{{ t("envGroup.context-menu.apply") }}</span>
					</DropdownMenuItem>
					<DropdownMenuItem @click="dropdownMenuDelete(props.data)">
						<Trash2 class="mr-2 h-4 w-4 text-destructive" />
						<span class="text-destructive">{{ t("envGroup.context-menu.delete") }}</span>
					</DropdownMenuItem>
				</DropdownMenuContent>
			</DropdownMenu>
		</div>
	</div>
	<div class="grid grid-flow-row items-center gap-1" v-if="showItems">
		<ItemEnv v-for="env in props.data.envs" :configId="props.data.configId" :data="env" @callback="emit('callback')"
			@remove="removeEnv(env.key)"></ItemEnv>
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
import type { GroupEnv } from "@/store/type";
import { invoke } from "@tauri-apps/api/core";
import {
	Ellipsis,
	Laugh,
	PanelBottomClose,
	PanelBottomOpen,
	Pencil,
	PlusSquare,
	SearchCheck,
	Trash2,
} from "lucide-vue-next";
import { defineEmits, defineProps, ref } from "vue";
import { useI18n } from "vue-i18n";
import { EditGroupEnv, EditItemEnv } from ".";
import { ItemEnv } from ".";

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

const showItems = ref(false);

const editItemEnvCallback = () => {
	showItems.value = true;
	emit("callback");
};
const removeEnv = (envKey: string) => {
	emit("removeEnv", props.data.configId, props.data.id, envKey);
};

// 检查
const dropdownMenuCheck = async (data: GroupEnv) => {
	// todo
	await invoke("group_env_check");
};

// 应用
const dropdownMenuApply = async (data: GroupEnv) => {
	// todo
	await invoke("group_env_apply");
};

// 删除
const dropdownMenuDelete = (data: GroupEnv) => {
	emit("remove", data.configId, data.id);
};
</script>