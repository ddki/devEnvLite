<template>
	<header class="flex flex-row justify-between pl-2 pr-2">
		<button type="button" class="button" @click="onCollate">{{ t('header.collate') }}</button>
		<button type="button" class="button" @click="openSetting">{{ t('header.setting') }}</button>
	</header>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const onCollate = () => { }
const openSetting = () => { }
</script>
