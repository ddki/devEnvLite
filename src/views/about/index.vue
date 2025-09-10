<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Button variant="outline">
        {{ t('common.about') }}
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent>
      <DropdownMenuItem @click="openLink('https://github.com/ddki/devEnvLite')">
        <span>GitHub</span>
      </DropdownMenuItem>
      <DropdownMenuItem @click="openLink('https://github.com/ddki/devEnvLite/wiki')">
        <span>{{ t("common.help-wiki") }}</span>
      </DropdownMenuItem>
      <DropdownMenuItem @click="openLink('https://github.com/ddki/devEnvLite/issues')">
        <span>{{ t("common.issue") }}</span>
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { openUrl } from "@tauri-apps/plugin-opener";

import { useI18n } from "vue-i18n";

const { t } = useI18n();

const openLink = async (url: string) => {
	await openUrl(url);
};
</script>