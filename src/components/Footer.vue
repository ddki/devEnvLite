<template>
	<footer class="grid grid-flow-col gap-4 justify-center items-center text-sm">
		<span class="grid grid-flow-col gap-1 items-center">
			<Tag class="h-4 w-4" />
			<span>V{{ appVersion }}</span>
		</span>
		<span class="grid grid-flow-col gap-1 items-center">
			<Copyright class="h-4 w-4" />
			<a target="_blank" href="https://www.gnu.org/licenses/gpl-3.0.html">GPL-3.0</a>
		</span>
	</footer>
</template>

<script setup lang="ts">
import { getVersion } from "@tauri-apps/api/app";
import { Copyright, Tag } from "lucide-vue-next";
// import { check } from "@tauri-apps/plugin-updater";
// import { relaunch } from "@tauri-apps/plugin-process";

const appVersion = await getVersion();

// const update = await check();

// if (update?.available) {
//   await update.downloadAndInstall();
//   await relaunch();
// }
</script>
