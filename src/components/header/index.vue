<template>
	<header class="flex flex-row justify-between p-2">
		<div class="flex flex-row items-start gap-2">
			<Collate />
			<Backup />
			<Recover />
			<a href="/splashscreen">splashscreen</a>
		</div>
		<div class="flex flex-row items-start gap-2">
			<Setting />
		</div>
	</header>

</template>

<script setup lang="ts">
import Setting from "@/components/setting/index.vue";
import { Backup, Collate, Recover } from "./index";
</script>
