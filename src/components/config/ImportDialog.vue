<script setup lang="ts">
import { LocalFileInput } from "@/components/common";
import { Button } from "@/components/ui/button";
import {
	Card,
	CardContent,
	CardDescription,
	CardFooter,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
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
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { generateConfigFromEnvs } from "@/store/config";
import type { Config } from "@/store/type";
import { invoke } from "@tauri-apps/api/core";
import { Import } from "lucide-vue-next";
import { v4 as uuidv4 } from "uuid";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useToast } from "@/components/ui/toast";
import { validateUrl } from "@/utils/ValidateUtil";

const { t } = useI18n();
const { toast } = useToast();

const scopesList = [
	{ label: t("env.scopes.user"), value: "USER" },
	{ label: t("env.scopes.system"), value: "SYSTEM" },
];

const emit = defineEmits(["callback"]);

const dialogOpen = ref(false);

const systemConfigName = ref("");
const systemScope = ref("USER");
const fileConfigName = ref("");
const urlConfigName = ref("");
const filePath = ref("");
const url = ref("");

const init = () => {
	systemConfigName.value = "";
	fileConfigName.value = "";
	urlConfigName.value = "";
	filePath.value = "";
	url.value = "";
};

const importFromSystem = async () => {
	console.log("importFromSystem, systemConfigName: ", systemConfigName);
	if (!systemScope.value || systemScope.value.length < 0) {
		toast({
			title: t("config.import-config.types.env.text"),
			description: t("env.error.checkScope"),
			variant: "destructive",
		});
		return;
	}
	if (!systemConfigName.value || systemConfigName.value.length < 0) {
		toast({
			title: t("config.import-config.types.env.text"),
			description: t("config.error.nameNotEmpty"),
			variant: "destructive",
		});
		return;
	}
	await invoke("get_envs", { scope: systemScope.value })
		.then(async (res) => {
			if (res) {
				const resMap = new Map<string, string>(Object.entries(res));
				console.log(typeof resMap);
				const config: Config = {
					id: uuidv4(),
					scope: systemScope.value,
					name: systemConfigName.value,
					note: `${t("config.import-config.text")}-${t("config.import-config.types.env.text")}`,
					sort: 1,
				};
				await generateConfigFromEnvs(config, resMap)
					.then(() => {
						emit("callback");
						toast({
							title: t("config.import-config.types.env.text"),
							description: `${t("config.import-config.text")}-${t(
								"config.import-config.types.env.text",
							)}: ${t("message.success")}`,
						});
					})
					.catch((err) => {
						console.error(err);
						toast({
							title: t("config.import-config.types.env.text"),
							description: `${t("message.error")} : ${err.message}`,
							variant: "destructive",
						});
					});
			}
		})
		.catch((err) => {
			console.error(err);
			toast({
				title: t("config.import-config.types.env.text"),
				description: `${t("message.error")} : ${err.message}`,
				variant: "destructive",
			});
		});
};
const importFromFile = async () => {
	if (!fileConfigName.value || fileConfigName.value.length < 0) {
		toast({
			title: t("config.import-config.types.file.text"),
			description: `${t("message.field-not-empty", { field: t("config.import-config.types.file.name") })}`,
			variant: "destructive",
		});
		return;
	}
	if (!filePath.value || filePath.value.length < 0) {
		toast({
			title: t("config.import-config.types.file.text"),
			description: `${t("message.field-not-empty", { field: t("config.import-config.types.file.file") })}`,
			variant: "destructive",
		});
		return;
	}

	await invoke("import_config_form_file", { path: filePath.value, name: fileConfigName.value })
		.then(async () => {
			emit("callback");
			toast({
				title: `${t("config.import-config.text")} t("config.import-config.types.file.text")`,
				description: `${t("config.import-config.text")}-${t(
					"config.import-config.types.file.text",
				)}: ${t("message.success")}`,
			});
		})
		.catch((err) => {
			toast({
				title: `${t("config.import-config.text")} t("config.import-config.types.file.text")`,
				description: `${t("message.error")} : ${err.message}`,
				variant: "destructive",
			});
		});
};
const importFromUrl = async () => {
	if (!urlConfigName.value || urlConfigName.value.length < 0) {
		toast({
			title: t("config.import-config.types.url.text"),
			description: `${t("message.field-not-empty", { field: t("config.import-config.types.url.name") })}`,
			variant: "destructive",
		});
		return;
	}
	if (!url.value || url.value.length < 0) {
		toast({
			title: t("config.import-config.types.url.text"),
			description: `${t("message.field-not-empty", { field: t("config.import-config.types.url.url") })}`,
			variant: "destructive",
		});
		return;
	}
	if(!validateUrl(url.value)) {
		toast({
			title: t("config.import-config.types.url.text"),
			description: `${t("message.field-error-format", { field: t("config.import-config.types.url.url") })}`,
			variant: "destructive",
		});
		return;
	}

	await invoke("import_config_form_url", { url: url.value, name: urlConfigName.value })
		.then(async () => {
			emit("callback");
			toast({
				title: `${t("config.import-config.text")}-t("config.import-config.types.url.text")`,
				description: `${t("config.import-config.text")}-${t(
					"config.import-config.types.url.text",
				)}: ${t("message.success")}`,
			});
		})
		.catch((err) => {
			toast({
				title: `${t("config.import-config.text")}-t("config.import-config.types.url.text")`,
				description: `${t("message.error")} : ${err.message}`,
				variant: "destructive",
			});
		});
};

watch(dialogOpen, (newValue) => {
	if (!newValue) {
		init();
	}
});
</script>

<template>
	<Dialog v-model:open="dialogOpen">
		<DialogTrigger as-child>
			<Button variant="outline" @click="dialogOpen = true">
				<Import class="mr-2 h-6 w-6" />
				{{ t('config.import-config.text') }}
			</Button>
		</DialogTrigger>
		<DialogContent>
			<DialogHeader>
				<DialogTitle>{{ t('config.import-config.text') }}</DialogTitle>
				<DialogDescription>
					{{ t('config.import-config.description') }}
				</DialogDescription>
				<DialogClose />
			</DialogHeader>
			<Tabs default-value="system">
				<TabsList class="grid w-full grid-cols-3">
					<TabsTrigger value="system">
						{{ t('config.import-config.types.env.text') }}
					</TabsTrigger>
					<TabsTrigger value="file">
						{{ t('config.import-config.types.file.text') }}
					</TabsTrigger>
					<TabsTrigger value="url">
						{{ t('config.import-config.types.url.text') }}
					</TabsTrigger>
				</TabsList>
				<TabsContent value="system">
					<Card>
						<CardHeader class="items-center">
							<CardTitle>{{ t('config.import-config.types.env.text') }}</CardTitle>
							<CardDescription>
								{{ t('config.import-config.types.env.description') }}
							</CardDescription>
						</CardHeader>
						<CardContent class="space-y-2">
							<div class="grid grid-cols-4 items-center gap-2">
								<Label for="name" class="text-right">{{ t('config.import-config.types.env.scope') }}</Label>
								<RadioGroup v-model="systemScope">
									<div class="flex items-center space-x-2" v-for="item in scopesList" :key="item.value">
										<RadioGroupItem :value="item.value" />
										<Label for="r1">{{ item.label }}</Label>
									</div>
								</RadioGroup>
							</div>
							<div class="grid grid-cols-4 items-center gap-2">
								<Label for="name" class="text-right">{{ t('config.import-config.types.env.name') }}</Label>
								<Input v-model="systemConfigName" class="col-span-3" />
							</div>
						</CardContent>
						<CardFooter class="justify-center">
							<Button @click="importFromSystem">
								{{ t('config.import-config.text') }}
							</Button>
						</CardFooter>
					</Card>
				</TabsContent>

				<TabsContent value="file">
					<Card>
						<CardHeader class="items-center">
							<CardTitle>{{ t('config.import-config.types.file.text') }}</CardTitle>
							<CardDescription>
								{{ t('config.import-config.types.file.description') }}
							</CardDescription>
						</CardHeader>
						<CardContent class="space-y-2">
							<div class="grid grid-cols-4 items-center gap-2">
								<Label for="name" class="text-right">{{ t('config.import-config.types.file.name') }}</Label>
								<Input v-model="fileConfigName" type="text" class="col-span-3" />
							</div>
							<div class="grid grid-cols-4 items-center gap-2">
								<Label for="name" class="text-right">{{ t('config.import-config.types.file.file') }}</Label>
								<LocalFileInput v-model="filePath" type="file" :placeholder="t('config.import-config.types.file.file')"
									class="col-span-3" />
							</div>
						</CardContent>
						<CardFooter class="justify-center">
							<Button @click="importFromFile">
								{{ t('config.import-config.text') }}
							</Button>
						</CardFooter>
					</Card>
				</TabsContent>

				<TabsContent value="url">
					<Card>
						<CardHeader class="items-center">
							<CardTitle>{{ t('config.import-config.types.url.text') }}</CardTitle>
							<CardDescription>
								{{ t('config.import-config.types.url.description') }}
							</CardDescription>
						</CardHeader>
						<CardContent class="space-y-2">
							<div class="grid grid-cols-4 items-center gap-2">
								<Label for="name" class="text-right">{{ t('config.import-config.types.url.name') }}</Label>
								<Input v-model="urlConfigName" type="text" class="col-span-3" />
							</div>
							<div class="grid grid-cols-4 items-center gap-2">
								<Label for="name" class="text-right">{{ t('config.import-config.types.url.url') }}</Label>
								<Input v-model="url" type="url" class="col-span-3" />
							</div>
						</CardContent>
						<CardFooter class="justify-center">
							<Button @click="importFromUrl">
								{{ t('config.import-config.text') }}
							</Button>
						</CardFooter>
					</Card>
				</TabsContent>
			</Tabs>
			<DialogFooter>
			</DialogFooter>
		</DialogContent>
	</Dialog>
</template>
