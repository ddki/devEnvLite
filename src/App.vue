<template>
	<Header />
	<main class="grid grid-cols-main">
		<div class="bg-slate-400">
			left
		</div>
		<div class="bg-slate-500">
			center
		</div>
		<div class="bg-slate-600">
			right
		</div>
	</main>

	<Suspense>
		<template #default>
			<Footer />
		</template>
	</Suspense>
</template>


<script setup lang="ts">
import { defineAsyncComponent, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Header from "./components/Header.vue";

onMounted(() => {
	invoke("close_splashscreen");
});

const Footer = defineAsyncComponent(() => import("./components/Footer.vue"));
</script>
