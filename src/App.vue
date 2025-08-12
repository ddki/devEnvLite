<template>
	<Suspense>
		<div class="h-dvh">
			<div class="h-full grid grid-rows-[3.5rem_1fr_1.5rem] grid-cols-1 overflow-hidden">
				<Header />
				<main class="border h-full overflow-hidden">
					<ResizablePanelGroup direction="horizontal">
						<ResizablePanel :default-size="30" class="h-full">
							<Config v-model:model-value="currentConfig" />
						</ResizablePanel>
						<ResizableHandle />
						<ResizablePanel :default-size="70" class="h-full">
							<EnvironmentPanel v-if="currentConfig.id" v-model:config="currentConfig" />
						</ResizablePanel>
					</ResizablePanelGroup>
				</main>
				<Footer />
			</div>
			<Toaster />
		</div>
	</Suspense>
</template>


<script setup lang="ts">
import { Toaster } from "@/components/ui/sonner";
import "vue-sonner/style.css";
import { Footer } from "@/components/footer";
import { Header } from "@/components/header";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { defineAsyncComponent, ref, watch } from "vue";
import type { EnvConfig } from "./types";

const Config = defineAsyncComponent({
	loader: () => import("@/views/config/index.vue"),
	errorComponent: () => import("@/components/common/ComponentError.vue"),
});

const EnvironmentPanel = defineAsyncComponent({
	loader: () => import("@/views/environment/index.vue"),
	errorComponent: () => import("@/components/common/ComponentError.vue"),
});

const currentConfig = ref<EnvConfig>({} as EnvConfig);

watch(currentConfig, (newVal) => {
	console.log("selectedConfig: ", newVal);
});
</script>
