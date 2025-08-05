<template>
	<Suspense>
		<div class="grid grid-rows-[3.5rem_1fr_1.5rem] grid-cols-1 h-dvh overflow-hidden">
			<Header />
			<main class="border overflow-auto">
				<ResizablePanelGroup direction="horizontal">
					<ResizablePanel :default-size="30">
						<Config v-model:currentConfigId="currentConfigId" />
					</ResizablePanel>
					<ResizableHandle />
					<ResizablePanel :default-size="70">
						<GroupEnv :configId="currentConfigId" />
					</ResizablePanel>
				</ResizablePanelGroup>
			</main>
			<Footer />
			<Toaster />
		</div>
	</Suspense>
</template>


<script setup lang="ts">
import { Toaster } from "@/components/ui/sonner";
import { Footer } from "@/components/footer";
import { Header } from "@/components/header";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { defineAsyncComponent, ref } from "vue";

const Config = defineAsyncComponent({
	loader: () => import("@/views/config/index.vue"),
	errorComponent: () => import("@/components/common/ComponentError.vue"),
});

const GroupEnv = defineAsyncComponent({
	loader: () => import("@/views/groupenv/index.vue"),
	errorComponent: () => import("@/components/common/ComponentError.vue"),
});

const currentConfigId = ref("");
</script>
