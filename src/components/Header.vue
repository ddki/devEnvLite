<template>
	<header class="flex flex-row justify-between p-2">
		<div class="flex flex-row items-start gap-2">
			<TooltipProvider>
				<Tooltip>
					<TooltipTrigger>
						<Button @click="onCollate">{{ t('header.collate') }}</Button>
					</TooltipTrigger>
					<TooltipContent>
						{{ t('header.description.collate') }}
					</TooltipContent>
				</Tooltip>
			</TooltipProvider>
			<TooltipProvider>
				<Tooltip>
					<TooltipTrigger>
						<Button @click="onBackup">{{ t('header.backup') }}</Button>
					</TooltipTrigger>
					<TooltipContent>
						{{ t('header.description.backup') }}
					</TooltipContent>
				</Tooltip>
			</TooltipProvider>
			<TooltipProvider>
				<Tooltip>
					<TooltipTrigger>
						<Button @click="onRecover">{{ t('header.recover') }}</Button>
					</TooltipTrigger>
					<TooltipContent>
						{{ t('header.description.recover') }}
					</TooltipContent>
				</Tooltip>
			</TooltipProvider>
		</div>
		<div class="flex flex-row items-start gap-2">
			<Setting />
		</div>
	</header>

</template>

<script setup lang="ts">
import {
	Tooltip,
	TooltipContent,
	TooltipProvider,
	TooltipTrigger
} from '@/components/ui/tooltip'
import Setting from "@/components/setting/index.vue";
import { Button } from "@/components/ui/button";
import { useToast } from "@/components/ui/toast/use-toast";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const { toast } = useToast();

const onCollate = async () => {
	await invoke("collate_envs");
	toast({
		title: "一键整理",
		description: "哈哈哈哈哈哈哈哈哈哈哈哈",
	});
};

const onBackup = async () => {
	// todo
	await invoke("backup_envs");
};

const onRecover = async () => {
	// todo
	await invoke("recover_envs");
};
</script>
