<template>
	<header class="flex flex-row justify-between p-2">
		<div>
			<el-button type="primary" @click="onCollate">{{ t('header.collate') }}</el-button>
			<el-button type="primary" @click="onCollate">{{ t('header.backup') }}</el-button>
			<el-button type="primary" @click="onCollate">{{ t('header.recover') }}</el-button>
		</div>
		<div>
			<el-button type="primary" @click="openSettings">{{ t('header.setting') }}</el-button>
			<SettingModal v-model:visible="settingsModalVisible" :title="t('header.setting')" />
		</div>
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
