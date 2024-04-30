<template>
	<Suspense>
		<div class="grid grid-rows-[3.5rem_1fr_1.5rem] grid-cols-1 h-dvh overflow-hidden">
			<Header />
			<main class="border overflow-auto">
				<ResizablePanelGroup direction="horizontal">
					<ResizablePanel :default-size="30">
						<Config v-model:activeConfigId="activeConfigId" v-model:selectedConfigId="selectedConfigId" />
					</ResizablePanel>
					<ResizableHandle />
					<ResizablePanel :default-size="70">
						<GroupEnv :configId="selectedConfigId" />
					</ResizablePanel>
				</ResizablePanelGroup>
			</main>
			<Footer />
			<Toaster />
		</div>
	</Suspense>
</template>


<script setup lang="ts">
import { Footer } from "@/components/footer";
import { Header } from "@/components/header";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { Toaster, useToast } from "@/components/ui/toast";
import { getActiveConfig } from "@/store/index";
import Config from "@/views/config/index.vue";
import GroupEnv from "@/views/groupenv/index.vue";
import { invoke } from "@tauri-apps/api/core";
import { onBeforeMount, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const { toast } = useToast();

const activeConfigId = ref("");
const selectedConfigId = ref("");


onMounted(async () => {
	const activeConfig = await getActiveConfig();
	if (activeConfig?.activeConfigId && activeConfig.activeConfigId.length > 0) {
		activeConfigId.value = activeConfig.activeConfigId;
		selectedConfigId.value = activeConfigId.value;
		await invoke("config_check", { configId: activeConfig.activeConfigId })
		.then(() => {
			toast({
				title: `${t("operate.check")} ${t("config.text")}`,
				description: t("message.success"),
			});
		})
		.catch((e) => {
			toast({
				title: `${t("operate.check")} ${t("config.text")}`,
				description: `${t("message.error")}: ${e.message}`,
				variant: "destructive",
			});
			console.log("application startup config_check error: ", e);
		});
	}
});

onBeforeMount(async () => {
	await invoke("close_splashscreen");
});

watch(selectedConfigId, (newValue, oldValue) => {
	console.log("selectedConfigId changed from ", oldValue, " to ", newValue);
});

</script>
