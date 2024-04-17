<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { ListCheckbox } from "@/components/ui/checkbox";
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
import { invoke } from "@tauri-apps/api/core";
import { reactive, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "../ui/toast/use-toast";

interface CheckInfo {
	label: string;
	value: string;
	checked: boolean;
}

const { t } = useI18n();

const open = ref(false);

const scopes = ref([]);
const keys = ref([]);
const keyList = ref<CheckInfo[]>([]);

let scopesList: CheckInfo[] = reactive<CheckInfo[]>([
	{ label: t("header.collate.env-scopes.user"), value: "USER", checked: false },
	{ label: t("header.collate.env-scopes.system"), value: "SYSTEM", checked: false },
]);

const init = () => {
	scopesList = [
		{ label: t("header.collate.env-scopes.user"), value: "USER", checked: false },
		{ label: t("header.collate.env-scopes.system"), value: "SYSTEM", checked: false },
	];
	keyList.value = [];
};

const onCollate = async () => {
	if (scopes.value.length === 0) {
		toast({
			title: t("header.collate.text"),
			description: t("header.collate.error.scopesNotEmpty"),
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

watch(
	scopes,
	async (newValue, oldValue) => {
		console.log("scopes == ", newValue, oldValue);
		if (scopes.value.length !== 0) {
			await invoke("get_keys", { scopes: scopes.value })
				.then((res) => {
					console.log("get_keys: ", res);
					if (res) {
						keyList.value = (res as string[]).map((item) => {
							return { label: item, value: item, checked: false };
						});
					}
					console.log("keyList: ", keyList);
				})
				.catch((err) => {
					console.error(err);
				});
		} else {
			keyList.value = [];
		}
	},
	{
		deep: true,
	},
);

watch(open, (newValue) => {
	if (!newValue) {
		init();
	}
});
</script>

<template>
	<Dialog v-model:open="open">
		<DialogTrigger as-child>
			<Button @click="open = true">{{ t('header.collate.text') }}</Button>
		</DialogTrigger>
		<DialogContent class="grid-rows-[auto_minmax(0,1fr)_auto] max-h-[90dvh]">
			<DialogHeader>
				<DialogTitle>{{ t('header.collate.text') }}</DialogTitle>
				<DialogDescription>
					{{ t('header.collate.description') }}
				</DialogDescription>
			</DialogHeader>
			<div class="grid gap-4 py-4 overflow-y-auto">
				<div class="grid grid-flow-row gap-4">
					<Label for="name" class="text-left text-base font-bold">
						{{ t("header.collate.env-scope.text") }}
						<span class="text-sm text-muted-foreground font-normal">
							{{ t("header.collate.env-scope.description") }}
						</span>
					</Label>
					<ListCheckbox :items="scopesList" v-model="scopes" class="pl-4" />
				</div>
				<div class="grid grid-flow-row gap-4">
					<Label for="username" class="text-left text-base font-bold">
						{{ t("header.collate.env-keys.text") }}
						<span class="text-sm text-muted-foreground font-normal">
							{{ t("header.collate.env-keys.description") }}
						</span>
					</Label>

					<ListCheckbox :items="keyList" v-model="keys" class="pl-4" />
				</div>
			</div>
			<DialogFooter>
				<Button @click="onCollate">
					{{ t('header.collate.text') }}
				</Button>
			</DialogFooter>
		</DialogContent>
	</Dialog>
</template>
