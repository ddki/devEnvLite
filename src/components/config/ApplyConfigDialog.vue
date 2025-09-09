<template>
	<Dialog v-model:open="open">
		<DialogTrigger as-child>
			<Button @click="open = true" variant="outline">
				<CircleCheckBig />
				{{ t('config.apply.text') }}
			</Button>
		</DialogTrigger>
		<DialogContent>
			<DialogHeader>
				<DialogTitle>{{ t('config.apply.text') }}</DialogTitle>
				<DialogDescription>
					{{ t('config.apply.description') }}
				</DialogDescription>
				<DialogClose />
			</DialogHeader>
			<div class="text-destructive-foreground font-black">
				{{ t('config.apply.warning') }}
			</div>
			<DialogFooter>
				<Button @click="onApply">
					{{ t('config.apply.text') }}
				</Button>
			</DialogFooter>
		</DialogContent>
	</Dialog>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
	Dialog,
	DialogClose,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from "@/components/ui/dialog";
import type { Res } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { CircleCheckBig } from "lucide-vue-next";
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

const { t } = useI18n();

const open = ref(false);

const onApply = async () => {
	const title = t("config.apply.text");
	await invoke<Res<void>>("apply_env_config")
		.then((res) => {
			if (res.code === "200") {
				toast.success(title, {
					description: t("message.success"),
				});
				console.log("Apply res = ", res);
			} else {
				toast.error(title, {
					description: `${t("message.error")} : ${res.message}`,
				});
			}
		})
		.catch((err) => {
			toast.error(title, {
				description: `${t("message.error")} : ${err.message}`,
			});
		})
		.finally(() => {
			open.value = false;
		});
};
</script>


