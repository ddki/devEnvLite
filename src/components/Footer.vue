<template>
	<footer class="flex flex-row justify-between pl-2 pr-2 bg-second text-slate-200 text-xs">
		<span>Version: <span>V{{ appVersion }}</span></span>
		<span>License: <a _blank href="https://www.gnu.org/licenses/gpl-3.0.html">GPL-3.0</a></span>
		<span>Author: <a _blank href="https://github.com/ddki">ddki</a></span>
		<span>Power by: <a _blank href="https://tauri.app/">Tauri</a></span>
	</footer>
</template>

<script setup lang="ts">
import { getVersion } from '@tauri-apps/api/app'
const appVersion = await getVersion()
</script>
