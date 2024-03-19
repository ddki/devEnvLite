<template>
	<main class="grid grid-cols-[30dvw_minmax(40dvh,_1fr)] p-2 sm:gap-x-2 md:gap-x-4">
		<Config />
		<GroupEnv />
	</main>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { getActiveConfig, getConfig } from "../store/config";
import Config from "./config/index.vue";
import GroupEnv from "./groupenv/index.vue";

const { t } = useI18n();
const activeConfig = await getActiveConfig();
const config = await getConfig("1");
</script>
