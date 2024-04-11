<template>
	<div class="grid">
		<div class="grid grid-flow-col gap-2 items-center">
			<TerminalSquare class="h-4 w-4" />
			<span>{{ props.data.key }}</span>
		</div>
		<div>
			<EditItemEnv operate="edit" :configId="props.configId" :groupId="props.data.groupId" :key="props.data.key" @callback="emit('callback')">
				<Button variant="ghost" size="icon">
					<Pencil class="h-4 w-4" />
				</Button>
			</EditItemEnv>
			<DropdownMenu>
				<DropdownMenuTrigger>
					<Button variant="ghost" size="icon">
						<Ellipsis class="h-4 w-4" />
					</Button>
				</DropdownMenuTrigger>
				<DropdownMenuContent>
					<DropdownMenuItem @click="dropdownMenuApply(props.data)">
						<Laugh class="mr-2 h-4 w-4" />
						<span>{{ t("config.context-menu.apply") }}</span>
					</DropdownMenuItem>
					<DropdownMenuItem @click="dropdownMenuDelete(props.data)">
						<Trash2 class="mr-2 h-4 w-4" />
						<span>{{ t("config.context-menu.delete") }}</span>
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
import { TerminalSquare,Pencil, Ellipsis, Laugh, Trash2 } from "lucide-vue-next";
import type { Env } from "@/store/type";
import { useI18n } from "vue-i18n";
import { EditItemEnv } from ".";

interface Props {
	configId: string,
	data: Env
}
const props = defineProps<Props>();
const emit = defineEmits(["callback", "remove"]);

const { t } = useI18n();

// 检查
const dropdownMenuCheck = (data: Env) => { };

// 应用
const dropdownMenuApply = (data: Env) => { };

// 删除
const dropdownMenuDelete = (data: Env) => {
	emit("remove", {key: data.key});
};
</script>
