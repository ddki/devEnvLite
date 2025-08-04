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
		</div>
	</Suspense>
</template>


<script setup lang="ts">
import { Footer } from "@/components/footer";
import { Header } from "@/components/header";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { getActiveConfig } from "@/store/index";
import { invoke } from "@tauri-apps/api/core";
import { defineAsyncComponent, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

const Config = defineAsyncComponent({
	loader: () => import("@/views/config/index.vue"),
	errorComponent: () => import("@/components/common/ComponentError.vue"),
});

const GroupEnv = defineAsyncComponent({
	loader: () => import("@/views/groupenv/index.vue"),
	errorComponent: () => import("@/components/common/ComponentError.vue"),
});
const { t } = useI18n();

const activeConfigId = ref("");
const selectedConfigId = ref("");

onMounted(async () => {
	const activeEnvConfigs = await invoke("list_active_env_configs")
		.then((res) => res.data)
		.catch((e) => {
			toast({
				title: `${t("operate.check")} ${t("config.text")}`,
				description: `${t("message.error")}: ${e.message}`,
				variant: "destructive",
			});
			console.error("Failed to load active environment configs: ", e);
		});
	console.log("application startup activeEnvConfigs: ", activeEnvConfigs);
	// if (activeConfig?.activeConfigId && activeConfig.activeConfigId.length > 0) {
	//     activeConfigId.value = activeConfig.activeConfigId;
	//     selectedConfigId.value = activeConfigId.value;
	//     await invoke("config_check", { configId: activeConfig.activeConfigId })
	//         .then(() => {
	//             toast({
	//                 title: `${t("operate.check")} ${t("config.text")}`,
	//                 description: t("message.success"),
	//             });
	//         })
	//         .catch((e) => {
	//             toast({
	//                 title: `${t("operate.check")} ${t("config.text")}`,
	//                 description: `${t("message.error")}: ${e.message}`,
	//                 variant: "destructive",
	//             });
	//             console.log("application startup config_check error: ", e);
	//         });
	// }
});

watch(selectedConfigId, (newValue, oldValue) => {
	console.log("selectedConfigId changed from ", oldValue, " to ", newValue);
});
</script>
