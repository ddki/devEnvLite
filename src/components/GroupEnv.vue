<template>
	<div class="grid sm:grid-rows-[2rem_minmax(0,_1fr)] md:grid-rows-[3rem_minmax(0,_1fr)] shadow-xl">
		<div class="flex flex-row flex-2 justify-start items-center">
			<button type="button" class="btn btn-primary btn-sm sm:btn-sm md:btn-md sm:mr-2 md:mr-4">
				{{ t('env-group.new-config-group') }}
			</button>
		</div>
		<div>
			<p>配置1</p>
		</div>
	</div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
</script>
