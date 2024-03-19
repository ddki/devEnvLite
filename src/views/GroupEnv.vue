<template>
	<div class="grid sm:grid-rows-[2rem_minmax(0,_1fr)] md:grid-rows-[3rem_minmax(0,_1fr)] overflow-auto">
		<div class="flex flex-row flex-2 justify-start items-center">
			<button type="button" class="btn btn-primary btn-sm sm:btn-sm md:btn-md sm:mr-2 md:mr-4">
				{{ t('env-group.new-config-group') }}
			</button>
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
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
</script>
