<template>
	<header class="flex flex-row justify-between p-2">
		<div class="flex flex-row items-start gap-2">
			<Button @click="onCollate">{{ t('header.collate') }}</Button>
			<Button @click="onBackup">{{ t('header.backup') }}</Button>
			<Button @click="onRecover">{{ t('header.recover') }}</Button>
		</div>
		<div class="flex flex-row items-start gap-2">
			<Setting />
		</div>
	</header>

</template>

<script setup lang="ts">
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
