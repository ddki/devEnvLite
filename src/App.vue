<template>
	<Suspense>
		<div class="grid grid-rows-[10dvh_86dvh_4dvh] grid-cols-1 h-dvh">
			<Header />
			<main class="border">
				<ResizablePanelGroup id="main-panel-group" direction="horizontal">
					<ResizablePanel id="main-panel-search" :default-size="25">
						<Config v-model:activeConfigId="activeConfigId" v-model:selectedConfigId="selectedConfigId" />
					</ResizablePanel>
					<ResizableHandle />
					<ResizablePanel id="main-panel-content-view" :default-size="75">
						<GroupEnv v-model:configId="selectedConfigId" />
					</ResizablePanel>
				</ResizablePanelGroup>
			</main>
			<Footer />
		</div>
	</Suspense>
</template>


<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { ResizablePanel, ResizableHandle, ResizablePanelGroup } from "@/components/ui/resizable";
import Header from "@/components/Header.vue";
import Footer from "@/components/Footer.vue";
import { getActiveConfig } from "@/store/index";
import Config from "@/views/config/index.vue";
import GroupEnv from "@/views/groupenv/index.vue";

const activeConfigId = ref("");
const selectedConfigId = ref("");

onMounted(async () => {
	const activeConfig = await getActiveConfig();
	activeConfigId.value = activeConfig.activeConfigId;
	selectedConfigId.value = activeConfigId.value;
});


invoke("close_splashscreen");
</script>
