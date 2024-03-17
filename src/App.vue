<template>
	<Header />

	<Suspense>
		<template #default>
			<Main />
		</template>
	</Suspense>
	<Suspense>
		<template #default>
			<Footer />
		</template>
	</Suspense>
</template>


<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { themeChange } from "theme-change";
import { defineAsyncComponent } from "vue";
import Header from "./views/Header.vue";
const Main = defineAsyncComponent(() => import("./views/MainContent.vue"));
const Footer = defineAsyncComponent(() => import("./views/Footer.vue"));

invoke("close_splashscreen");
themeChange(false);
</script>
