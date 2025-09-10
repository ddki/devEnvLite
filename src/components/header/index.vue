<template>
	<header class="flex flex-row justify-between p-2">
		<div class="flex flex-row items-start gap-2">
			<Collate />
			<Backup />
			<Recover />
		</div>
		<div class="flex flex-row items-start gap-2">
			<About />
			<Setting />
		</div>
	</header>

</template>

<script setup lang="ts">
import Setting from "@/components/setting/index.vue";
import { Backup, Collate, Recover, About } from "./index";
</script>
