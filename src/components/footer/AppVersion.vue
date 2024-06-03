<script setup lang="ts">
import { getVersion } from "@tauri-apps/api/app";
import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";
import { Tag } from "lucide-vue-next";
import { useI18n } from "vue-i18n";
import { toast } from "../ui/toast/use-toast";

const appVersion = await getVersion();

const { t } = useI18n();

const update = await check().catch((e) => {
	console.error("check update error: ", e);
	toast({
		title: t("footer.check-update"),
		description: t("footer.connect-failed"),
		variant: "destructive",
	});
});

const updateApp = async () => {
	if (update?.available) {
		await update.downloadAndInstall();
		await relaunch();
	}
};
</script>

<template>
	<span class="grid grid-flow-col gap-1 items-center">
		<Tag class="h-4 w-4" />
		<span>V{{ appVersion }}</span>
	</span>
	<span class="grid grid-flow-col gap-1 items-center" v-if="update?.available" @click="updateApp">
		<span>latest version is {{ update?.version }}</span>
	</span>
</template>
