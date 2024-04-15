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
import {
	Select,
	SelectContent,
	SelectGroup,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select";
import { invoke } from "@tauri-apps/api/core";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "../ui/toast/use-toast";

const { t } = useI18n();

const open = ref(false);

const scopes = ref();
const keys = ref();
let selectKeys: string[] = [];

const onCollate = async () => {
	await invoke("collate_envs", { keys: keys.value, scopes: scopes.value })
		.then((res) => {
			console.log("collate res = ", res);
		})
		.catch((err) => {
			console.error(err);
		});
	open.value = false;
	toast({
		title: t("header.collate.text"),
		description: "哈哈哈哈哈哈哈哈哈哈哈哈",
	});
};

watch(scopes, async (newValue) => {
	await invoke("get_keys", { scopes: newValue })
		.then((res) => {
			console.log("get_keys: ", res);
			selectKeys = res as string[];
		})
		.catch((err) => {
			console.error(err);
		});
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
					<Select v-model="scopes">
						<SelectTrigger>
							<SelectValue :placeholder="t('header.collate.env-scope')" />
						</SelectTrigger>
						<SelectContent class="w-full">
							<SelectGroup>
								<SelectItem value="USER">
									{{ t("header.collate.env-scopes.user") }}
								</SelectItem>
								<SelectItem value="SYSTEM">
									{{ t("header.collate.env-scopes.system") }}
								</SelectItem>
							</SelectGroup>
						</SelectContent>
					</Select>
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="username" class="text-right">
						{{ t("header.collate.env-keys") }}
					</Label>
					<Select multiple v-model="keys">
						<SelectTrigger>
							<SelectValue :placeholder="t('header.collate.env-keys')" />
						</SelectTrigger>
						<SelectContent>
							<SelectGroup>
								<SelectItem v-for="envKey in selectKeys" :value="envKey">
									{{ envKey }}
								</SelectItem>
							</SelectGroup>
						</SelectContent>
					</Select>
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
