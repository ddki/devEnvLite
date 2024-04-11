<template>
	<div class="grid">
		<div class="grid grid-flow-col gap-2 items-center">
			<Group class="h-4 w-4" />
			<span>{{ props.data.name }}</span>
			<div>
				<ItemEnv v-for="env in props.data.envs" :key="env.key" :configId="props.data.configId" :data="env"
					@callback="emit('callback')" @remove="removeEnv(env.key)"></ItemEnv>
			</div>
		</div>
		<div class="grid grid-flow-col items-center">
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
					<DropdownMenuItem @click="dropdownMenuAddEnv(props)">
						<PlusSquare class="mr-2 h-4 w-4" />
						<span>{{ t("envGroup.context-menu.addEnv") }}</span>
					</DropdownMenuItem>
					<DropdownMenuItem @click="dropdownMenuCheck(props)">
						<SearchCheck class="mr-2 h-4 w-4" />
						<span>{{ t("envGroup.context-menu.check") }}</span>
					</DropdownMenuItem>
					<DropdownMenuItem @click="dropdownMenuApply(props)">
						<Laugh class="mr-2 h-4 w-4" />
						<span>{{ t("envGroup.context-menu.apply") }}</span>
					</DropdownMenuItem>
					<DropdownMenuItem @click="dropdownMenuDelete(props)">
						<Trash2 class="mr-2 h-4 w-4" />
						<span>{{ t("envGroup.context-menu.delete") }}</span>
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
import { defineProps, defineEmits } from "vue";
import { Group, Pencil, Ellipsis, SearchCheck, Laugh, Trash2, PlusSquare } from "lucide-vue-next";
import type { GroupEnv } from "@/store/type";
import { useI18n } from "vue-i18n";
import { EditGroupEnv } from ".";
import { ItemEnv } from ".";

interface Props {
	data: GroupEnv
}
const props = defineProps<Props>();
const emit = defineEmits(["callback", "remove", "removeEnv"]);

const { t } = useI18n();

const removeEnv = (envKey: string) => {
	emit("removeEnv", { configId: props.data.configId, groupId: props.data.id, envKey: envKey })
}
// 添加环境变量
const dropdownMenuAddEnv = (data: GroupEnv) => { };

// 检查
const dropdownMenuCheck = (data: GroupEnv) => { };

// 应用
const dropdownMenuApply = (data: GroupEnv) => { };

// 删除
const dropdownMenuDelete = (data: GroupEnv) => {
	emit("remove", { id: data.id })
};

</script>
