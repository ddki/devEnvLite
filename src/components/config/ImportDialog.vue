<script setup lang="ts">
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
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Import } from "lucide-vue-next";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const open = ref(false);

const systemConfigName = ref("");
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

const importFromSystem = () => {};
const importFromFile = () => {};
const importFromUrl = () => {};

watch(open, (newValue) => {
	if (!newValue) {
		init();
	}
});
</script>

<template>
	<Dialog v-model:open="open">
		<DialogTrigger as-child>
			<Button variant="outline" @click="open = true">
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
								<Input v-model="fileConfigName" class="col-span-3" />
							</div>
							<div class="grid grid-cols-4 items-center gap-2">
								<Label for="name" class="text-right">{{ t('config.import-config.types.file.file') }}</Label>
								<Input v-model="filePath" class="col-span-3" />
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
								<Input v-model="urlConfigName" class="col-span-3" />
							</div>
							<div class="grid grid-cols-4 items-center gap-2">
								<Label for="name" class="text-right">{{ t('config.import-config.types.url.url') }}</Label>
								<Input v-model="url" class="col-span-3" />
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
