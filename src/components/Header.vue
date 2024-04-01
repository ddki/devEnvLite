<template>
	<header class="flex flex-row justify-between p-2">
		<div class="flex flex-row items-start gap-2">
			<Button @click="onCollate">{{ t('header.collate') }}</Button>
			<Button @click="onCollate">{{ t('header.backup') }}</Button>
			<Button @click="onCollate">{{ t('header.recover') }}</Button>
		</div>
		<div class="flex flex-row items-end gap-2">
			<Button @click="openSettings">{{ t('header.setting') }}</Button>
			<Setting />
			<SettingModal v-model:visible="settingsModalVisible" :title="t('header.setting')" @callBack="reloadPage" />
		</div>
	</header>

</template>

<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { useToast } from '@/components/ui/toast/use-toast'
import { defineAsyncComponent, ref } from "vue";
import { useI18n } from "vue-i18n";
import Setting from "@/components/setting/index.vue";
const SettingModal = defineAsyncComponent(() => import("./setting/SettingModal.vue"));


const { t } = useI18n();
const { toast } = useToast()
const settingsModalVisible = ref(false);

const onCollate = () => {
	toast({
		title: "一键整理",
		description: "哈哈哈哈哈哈哈哈哈哈哈哈"
	});
};

const openSettings = () => {
	settingsModalVisible.value = true;
};

const reloadPage = () => {
	console.log("reloadPage...")
	window.location.reload;
}
</script>
