<script setup lang="ts">
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
import { Label } from "@/components/ui/label";
import { ListCheckbox } from "@/components/ui/checkbox";
import { invoke } from "@tauri-apps/api/core";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "../ui/toast/use-toast";

interface CheckInfo {
	label: string;
	value: string;
}

const { t } = useI18n();

const open = ref(false);

const scopes = ref([]);
const keys = ref([]);
const keyList = ref<CheckInfo[]>([]);

const scopesList: CheckInfo[] = [
	{ label: t("header.collate.env-scopes.user"), value: "USER" },
	{ label: t("header.collate.env-scopes.system"), value: "SYSTEM" },
];

const onCollate = async () => {
	if (scopes.value.length === 0) {
		toast({
			title: t("header.collate.text"),
			description: t("header.collate.error.scopesNotEmpty"),
			variant: "destructive",
		});
		return;
	}
	if (keys.value.length === 0) {
		toast({
			title: t("header.collate.text"),
			description: t("header.collate.error.keysNotEmpty"),
			variant: "destructive",
		});
		return;
	}
	await invoke("collate_envs", { keys: keys.value, scopes: scopes.value })
		.then((res) => {
			console.log("collate res = ", res);
		})
		.catch((err) => {
			console.error(err);
		});
	open.value = false;
};

watch(scopes, async (newValue) => {
	console.log("scopes == ", newValue)
	if (newValue.length !== 0) {
		await invoke("get_keys", { scopes: newValue as string[] })
			.then((res) => {
				console.log("get_keys: ", res);
				if (res) {
					(res as string[]).map((item) => {
						keyList.value.push({ label: item, value: item });
					});
				}
				console.log("keyList: ", keyList);
			})
			.catch((err) => {
				console.error(err);
			});
	}
});
</script>

<template>
	<Dialog :open="open">
		<DialogTrigger as-child>
			<Button @click="open = true">{{ t('header.collate.text') }}</Button>
		</DialogTrigger>
		<DialogContent>
			<DialogHeader>
				<DialogTitle>{{ t('header.collate.text') }}</DialogTitle>
				<DialogDescription>
					{{ t('header.collate.description') }}
				</DialogDescription>
			</DialogHeader>
			<div class="grid gap-4 py-4">
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="name" class="text-right">
						{{ t("header.collate.env-scope") }}
					</Label>
					<ListCheckbox :items="scopesList" v-model="scopes" class="col-span-3" />
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="username" class="text-right">
						{{ t("header.collate.env-keys") }}
					</Label>
					<ListCheckbox :items="keyList" v-model="keys" class="col-span-3"/>
				</div>
			</div>
			<DialogFooter>
				<Button variant="secondary" @click="open = false">
					{{ t("close") }}
				</Button>
				<Button @click="onCollate">
					{{ t('header.collate.text') }}
				</Button>
			</DialogFooter>
		</DialogContent>
	</Dialog>
</template>
