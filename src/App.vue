<template>
	<Header />
	<main class="grid grid-cols-[30dvw_minmax(40dvh,_1fr)] px-2 sm:gap-x-2 md:gap-x-4">
		<div class="grid sm:grid-rows-[2rem_minmax(0,_1fr)] md:grid-rows-[3rem_minmax(0,_1fr)] shadow-xl">
			<div class="flex flex-row flex-2 justify-start items-center">
				<button type="button" class="btn btn-primary btn-sm sm:btn-sm md:btn-md sm:mr-2 md:mr-4">
					{{ t('config.import-config') }}
				</button>
				<button type="button" class="btn btn-primary btn-sm sm:btn-sm md:btn-md">
					{{ t('config.new-config') }}
				</button>
			</div>
			<div>
				<p>配置1</p>
			</div>
		</div>
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
	</main>

	<Suspense>
		<template #default>
			<Footer />
		</template>
	</Suspense>
</template>


<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { themeChange } from "theme-change";
import { defineAsyncComponent, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import Header from "./components/Header.vue";
const Footer = defineAsyncComponent(() => import("./components/Footer.vue"));

const { t } = useI18n();

onMounted(() => {
	invoke("close_splashscreen");
	themeChange(false);
});
</script>
