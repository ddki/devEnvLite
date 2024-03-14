<template>
	<Header />
	<main>

	</main>

	<Suspense>
		<template #default>
			<Footer />
		</template>
	</Suspense>
</template>


<script setup lang="ts">
import { defineAsyncComponent } from "vue";
import Header from "./components/Header.vue";

const Footer = defineAsyncComponent(() => import("./components/Footer.vue"));
</script>
