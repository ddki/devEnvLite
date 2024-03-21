<template>
	<main class="grid grid-cols-[25dvw_minmax(40dvh,_1fr)] p-2 sm:gap-x-2 md:gap-x-4">
		<Config v-model:activeConfigId="activeConfigId" v-model:selectedConfigId="selectedConfigId" />
		<GroupEnv v-model:configId="selectedConfigId" />
	</main>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { onUnmounted } from "vue";
import { getActiveConfig } from "../store/config";
import Config from "./config/index.vue";
import GroupEnv from "./groupenv/index.vue";

const activeConfigId = ref("");
const selectedConfigId = ref("");

onUnmounted(async () => {
	const activeConfig = await getActiveConfig();
	activeConfigId.value = activeConfig.activeConfigId;
	selectedConfigId.value = activeConfigId.value;
});
</script>
