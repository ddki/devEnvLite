<script setup lang="ts">
import { getVersion } from "@tauri-apps/api/app";
import { relaunch } from "@tauri-apps/plugin-process";
import { type Update, check } from "@tauri-apps/plugin-updater";
import { Tag } from "lucide-vue-next";
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

const appVersion = await getVersion();

const { t } = useI18n();

const update = ref<boolean>(false);
// biome-ignore lint/suspicious/noConfusingVoidType: <explanation>
const checkResult = ref<Update | null | void>(null);

const checkUpdate = async () => {
	await check()
		.then((res) => {
			console.log("check update result: ", res);
			checkResult.value = res;
			if (res?.currentVersion === res?.version) {
				update.value = true;
			}
		})
		.catch((e) => {
			console.error("check update error: ", e);
			toast.error(t("footer.check-update"), {
				description: t("footer.connect-failed"),
			});
			return;
		});
};

const updateApp = async () => {
	if (update.value) {
		await checkResult.value?.downloadAndInstall();
		await relaunch();
	}
};

onMounted(async () => {
	await checkUpdate();
});
</script>

<template>
	<span class="grid grid-flow-col gap-1 items-center">
		<Tag class="h-4 w-4" />
		<span>V{{ appVersion }}</span>
	</span>
	<span class="grid grid-flow-col gap-1 items-center" v-if="update" @click="updateApp">
		<span>latest version is {{ checkResult?.version }}</span>
	</span>
</template>
