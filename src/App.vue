<template>
	<Suspense>
		<div class="grid grid-rows-[3.5rem_1fr_2rem] grid-cols-1 h-dvh">
			<Header />
			<main class="border">
				<ResizablePanelGroup direction="horizontal" class="px-2">
					<ResizablePanel :default-size="25">
						<Config v-model:activeConfigId="activeConfigId" v-model:selectedConfigId="selectedConfigId" />
					</ResizablePanel>
					<ResizableHandle />
					<ResizablePanel :default-size="75">
						<GroupEnv v-model:configId="selectedConfigId" />
					</ResizablePanel>
				</ResizablePanelGroup>
			</main>
			<Footer />
		</div>
	</Suspense>
	<Toaster />
</template>


<script setup lang="ts">
import Footer from "@/components/Footer.vue";
import Header from "@/components/Header.vue";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { Toaster } from "@/components/ui/toast";
import { getActiveConfig } from "@/store/index";
import Config from "@/views/config/index.vue";
import GroupEnv from "@/views/groupenv/index.vue";
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";

const activeConfigId = ref("");
const selectedConfigId = ref("");

onMounted(async () => {
	const activeConfig = await getActiveConfig();
	activeConfigId.value = activeConfig.activeConfigId;
	selectedConfigId.value = activeConfigId.value;
});

invoke("close_splashscreen");
</script>
