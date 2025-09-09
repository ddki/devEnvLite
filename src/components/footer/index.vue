<template>
	<footer class="grid grid-flow-col gap-4 justify-center items-center text-sm">
		<AppVersion />
		<span class="grid grid-flow-col gap-1 items-center">
			<Copyright class="h-4 w-4" />
			<a target="_blank" href="https://www.gnu.org/licenses/gpl-3.0.html">GPL-3.0</a>
		</span>
	</footer>
</template>

<script setup lang="ts">
import { Copyright } from "lucide-vue-next";
import { defineAsyncComponent } from "vue";
const AppVersion = defineAsyncComponent(() => import("./AppVersion.vue"));
</script>
