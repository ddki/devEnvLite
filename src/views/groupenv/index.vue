<template>
	<div class="grid sm:grid-rows-[2rem_minmax(0,_1fr)] md:grid-rows-[3rem_minmax(0,_1fr)] overflow-auto">
		<div class="flex flex-row flex-2 justify-start items-center">
			<el-button type="primary" @click="newGroupEnv">{{ t('env-group.new-config-group') }}</el-button>
		</div>
		<div class="sm:mt-1 md:mt-2 overflow-auto">
			<p class="h-60">配置1</p>
			<p class="h-60">配置1</p>
			<p class="h-60">配置1</p>
			<p class="h-60">配置1</p>
			<p class="h-60">配置1</p>
		</div>
	</div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const newGroupEnv = () => {};
</script>
