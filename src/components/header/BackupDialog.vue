<template>
	<Dialog v-model:open="open">
		<DialogTrigger as-child>
			<Button @click="open = true">
				<HardDriveDownload />
				{{ t('header.backup.text') }}
			</Button>
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
					<Input v-model="name" :placeholder="t('header.backup.name')" class="col-span-3" />
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="username" class="text-right">
						{{ t("header.backup.folder") }}
					</Label>
					<LocalFileInput type="folder" v-model="folder" :placeholder="t('header.backup.folder')" class="col-span-3" />
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

<script setup lang="ts">
import { LocalFileInput } from "@/components/common";
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
import type { Res, Setting } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { HardDriveDownload } from "lucide-vue-next";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

const { t } = useI18n();

const open = ref(false);

const setting = await invoke<Res<Setting>>("get_settings")
	.then((res) => {
		if (res.code === "200") {
			return res.data;
		}
	})
	.catch(() => {
		toast.warning(t("header.setting"), {
			description: `${t("operate.save")}${t("message.failure")}`,
		});
	});
const defaultName = `${t("header.backup.text")}${t("env.text")}`;
const name = ref(defaultName);
const folder = ref(setting?.envBackupDir || "");

const init = () => {
	name.value = defaultName;
	folder.value = setting?.envBackupDir || "";
};

const onBackup = async () => {
	const title = t("header.backup.text");
	await invoke<Res<void>>("backup_os_environment_variables", { backupName: name.value })
		.then((res) => {
			if (res.code === "200") {
				toast.success(title);
			} else {
				toast.error(title, {
					description: `${t("message.error")} : ${res.message}`,
				});
			}
		})
		.catch((err) => {
			console.error(err);
			toast.error(title, {
				description: `${t("message.error")} : ${err.message}`,
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


