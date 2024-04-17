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
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { invoke } from "@tauri-apps/api/core";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useToast } from "../ui/toast";

const { t } = useI18n();
const { toast } = useToast();

const open = ref(false);

const name = ref("");
const folder = ref("");

const init = () => {
	name.value = "";
	folder.value = "";
};

const onBackup = async () => {
	await invoke("backup_envs", { backupName: name.value, folder: folder.value })
		.then((res) => {
			console.log("Backup res = ", res);
		})
		.catch((err) => {
			console.error(err);
			toast({
				title: t("config.import-config.types.env.text"),
				description: `${t("error")} : ${err.message}`,
				variant: "destructive",
			});
		});
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
			<Button @click="open = true">{{ t('header.backup.text') }}</Button>
		</DialogTrigger>
		<DialogContent>
			<DialogHeader>
				<DialogTitle>{{ t('header.backup.text') }}</DialogTitle>
				<DialogDescription>
					{{ t('header.backup.description') }}
				</DialogDescription>
				<DialogClose />
			</DialogHeader>
			<div class="grid gap-4 py-4">
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="name" class="text-right">
						{{ t("header.backup.name") }}
					</Label>
					<Input v-model="name" class="col-span-3" />
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="username" class="text-right">
						{{ t("header.backup.folder") }}
					</Label>
					<Input v-model="folder" class="col-span-3" />
				</div>
			</div>
			<DialogFooter>
				<Button @click="onBackup">
					{{ t('header.backup.text') }}
				</Button>
			</DialogFooter>
		</DialogContent>
	</Dialog>
</template>
