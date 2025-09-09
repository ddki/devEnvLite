<template>
	<Dialog v-model:open="open">
		<DialogTrigger as-child>
			<Button @click="open = true">
				<Undo2 />
				{{ t('header.recover.text') }}
			</Button>
		</DialogTrigger>
		<DialogContent>
			<DialogHeader>
				<DialogTitle>{{ t('header.recover.text') }}</DialogTitle>
				<DialogDescription>
					{{ t('header.recover.description') }}
					<div class="text-destructive-foreground font-black text-base">
						{{ t('header.recover.warning') }}
					</div>
				</DialogDescription>
			</DialogHeader>
			<div class="grid gap-4 py-4">
				<div class="grid grid-flow-row gap-4">
					<Label for="scope" class="text-left text-base font-bold">
						{{ t("env.scope.text") }}
						<span class="text-sm text-muted-foreground font-normal">
							{{ t("env.scope.description") }}
						</span>
					</Label>
					<RadioGroup v-model="scope" :orientation="'vertical'">
						<div class="flex items-center space-x-2" v-for="item in scopesList" :key="item.value">
							<RadioGroupItem :id="item.value" :value="item.value" />
							<Label :for="item.value">{{ item.label }}</Label>
						</div>
					</RadioGroup>
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="username" class="text-right">
						{{ t("header.recover.file") }}
					</Label>
					<LocalFileInput v-model="file" type="file" :placeholder="t('header.recover.file')"
						class="col-span-3" />
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
import { Label } from "@/components/ui/label";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { EnvironmentVariableScope, getEnvironmentVariableScopeList } from "@/constants";
import type { Res } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { Undo2 } from "lucide-vue-next";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

const { t } = useI18n();

const open = ref(false);

const scope = ref(EnvironmentVariableScope.USER);
const file = ref("");

const scopesList = getEnvironmentVariableScopeList(t);

const init = () => {
	scope.value = EnvironmentVariableScope.USER;
	file.value = "";
};

const onRecover = async () => {
	const title = t("header.recover.text");
	await invoke<Res<void>>("recover_os_environment_variables", {
		backupFile: file.value,
		scope: scope.value,
	})
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
