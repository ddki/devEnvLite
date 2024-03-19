<template>
	<header class="flex flex-row justify-between p-2">
		<el-button type="primary" @click="onCollate">{{ t('header.collate') }}</el-button>
		<el-button type="primary" @click="openSettings">{{ t('header.setting') }}</el-button>
		<SettingModal v-model:visible="settingsModalVisible" :title="t('header.setting')" />
	</header>

</template>

<script setup lang="ts">
import { ElNotification } from "element-plus";
import { defineAsyncComponent, ref } from "vue";
import { useI18n } from "vue-i18n";
const SettingModal = defineAsyncComponent(() => import("./setting/SettingModal.vue"));

const { t } = useI18n();
const settingsModalVisible = ref(false);

const onCollate = () => {
	ElNotification({
		title: "一键整理",
		message: "哈哈哈哈哈哈哈哈哈哈哈哈",
		position: "bottom-right",
	});
};

const openSettings = () => {
	settingsModalVisible.value = true;
};
</script>
