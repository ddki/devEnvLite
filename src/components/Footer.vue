<template>
	<footer class="flex flex-row justify-center items-center pl-2 pr-2 bg-secondary text-sm">
		<span class="mr-6">Version: <span>V{{ appVersion }}</span></span>
		<span class="mr-6">License: <a target="_blank" href="https://www.gnu.org/licenses/gpl-3.0.html">GPL-3.0</a></span>
		<span>Power by: <a target="_blank" href="https://tauri.app/">Tauri</a></span>
	</footer>
</template>

<script setup lang="ts">
import { getVersion } from '@tauri-apps/api/app'
// import { check } from "@tauri-apps/plugin-updater";
// import { relaunch } from "@tauri-apps/plugin-process";

const appVersion = await getVersion()

// const update = await check();

// if (update?.available) {
//   await update.downloadAndInstall();
//   await relaunch();
// }
</script>
