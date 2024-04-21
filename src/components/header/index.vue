<template>
	<header class="flex flex-row justify-between p-2">
		<div class="flex flex-row items-start gap-2">
			<Collate />
			<Backup />
			<Recover />
		</div>
		<div class="flex flex-row items-start gap-2">
			<ThemeChange :placeholder="t('header.theme')" />
			<Setting />
		</div>
	</header>

</template>

<script setup lang="ts">
import { ThemeChange } from "@/components/common";
import Setting from "@/components/setting/index.vue";
import { useI18n } from "vue-i18n";
import { Backup, Collate, Recover } from "./index";

const { t } = useI18n();
</script>
