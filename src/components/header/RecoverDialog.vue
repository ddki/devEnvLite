<script setup lang="ts">
import { LocalFileInput } from "@/components/common";
import { Button } from "@/components/ui/button";
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { invoke } from "@tauri-apps/api/core";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

const { t } = useI18n();

const open = ref(false);

const name = ref("");
const file = ref("");

const init = () => {
	name.value = "";
	file.value = "";
};

const onRecover = async () => {
	await invoke("recover_envs", { file: file.value, name: name.value })
		.then((res) => {
			console.log("recover res = ", res);
		})
		.catch((err) => {
			console.error(err);
			toast({
				title: t("config.import-config.types.env.text"),
				description: `${t("message.error")} : ${err.message}`,
				variant: "destructive",
			});
		});
	open.value = false;
};

watch(open, (newValue) => {
	if (!newValue) {
		init();
	}
});
</script>

<template>
	<Dialog v-model:open="open">
		<DialogTrigger as-child>
			<Button @click="open = true">{{ t('header.recover.text') }}</Button>
		</DialogTrigger>
		<DialogContent>
			<DialogHeader>
				<DialogTitle>{{ t('header.recover.text') }}</DialogTitle>
				<DialogDescription>
					{{ t('header.recover.description') }}
				</DialogDescription>
			</DialogHeader>
			<div class="grid gap-4 py-4">
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="name" class="text-right">
						{{ t("header.recover.name") }}
					</Label>
					<Input v-model="name" class="col-span-3" />
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="username" class="text-right">
						{{ t("header.recover.file") }}
					</Label>
					<LocalFileInput v-model="file" type="file" :placeholder="t('header.recover.file')" class="col-span-3" />
				</div>
			</div>
			<DialogFooter>
				<Button @click="onRecover">
					{{ t('header.recover.text') }}
				</Button>
			</DialogFooter>
		</DialogContent>
	</Dialog>
</template>
