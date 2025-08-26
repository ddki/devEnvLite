<template>
	<Dialog v-model:open="open">
		<DialogTrigger as-child>
			<Button @click="open = true">
				<Blocks />
				{{ t('header.collate.text') }}
			</Button>
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
					<CheckboxGroupRoot v-model="scopes" @update:model-value="onScopesChange">
						<div v-for="item in scopesList" :key="item.value" class="flex flex-row items-center gap-2 p-1">
							<Checkbox :id="item.value" :value="item.value" />
							<Label :for="item.value">{{ item.label }}</Label>
						</div>
					</CheckboxGroupRoot>
				</div>
				<div class="grid grid-flow-row gap-4">
					<Label for="username" class="text-left text-base font-bold">
						{{ t("header.collate.env-keys.text") }}
						<span class="text-sm text-muted-foreground font-normal">
							{{ t("header.collate.env-keys.description") }}
						</span>
					</Label>
					<CheckboxGroupRoot v-model="keys"
						@update:model-value="(values: string[]) => { console.log('group changed', values) }">
						<div v-for="item in keyList" :key="item.value" class="flex flex-row items-center gap-2 p-1">
							<Checkbox :id="item.value" :value="item.value" />
							<Label :for="item.value">{{ item.label }}</Label>
						</div>
					</CheckboxGroupRoot>
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

<script setup lang="ts">
import { CheckboxGroupRoot } from "reka-ui";
import { Checkbox } from "@/components/ui/checkbox";
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
import { getEnvironmentVariableScopeList } from "@/constants";
import type { Res } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { Blocks } from "lucide-vue-next";
import { reactive, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

interface CheckInfo {
	label: string;
	value: string;
	checked: boolean;
}

const { t } = useI18n();

const defaultScopes = (): CheckInfo[] => {
	return getEnvironmentVariableScopeList(t).map((item) => ({
		...item,
		checked: false,
	}));
};

const open = ref(false);

// 正确指定scopes的类型为string数组
const scopes = ref<string[]>([]);
const keys = ref([]);
const keyList = ref<CheckInfo[]>([]);

let scopesList: CheckInfo[] = reactive<CheckInfo[]>(defaultScopes());

const init = () => {
	scopesList = defaultScopes();
	keyList.value = [];
};

const onScopesChange = async (scopes: string[]) => {
	console.log("onScopesChange: ", scopes);
	const title = t("header.collate.text");
	// 确保传递给invoke的是字符串数组类型
	const scopesToPass = Array.isArray(scopes) ? scopes : scopes ? [scopes] : [];
	await invoke<Res<string[]>>("get_os_environment_variable_keys", { scopes: scopesToPass })
		.then((res) => {
			if (res.code === "200") {
				keyList.value = (res.data as string[]).map((item) => {
					return { label: item, value: item, checked: false };
				});
				return;
			}
			toast.error(title, {
				description: `${t("operate.query")}${t("env.text")}${t("message.error")} : ${res.message}`,
			});
		})
		.catch((err) => {
			console.error(title, err);
			toast.error(title, {
				description: `${t("operate.query")}${t("env.text")}${t("message.error")} : ${err.message}`,
			});
		});
};

const onCollate = async () => {
	const title = t("header.collate.text");
	if (scopes.value.length === 0) {
		toast.warning(title, {
			description: t("header.collate.error.scopesNotEmpty"),
		});
		return;
	}
	await invoke<Res<void>>("collate_os_environment_variables", {
		keys: keys.value,
		scopes: scopes.value,
	})
		.then((res) => {
			console.log("onCollate: ", res);
			if (res.code === "200") {
				toast.success(title);
			} else {
				toast.error(title, {
					description: `${t("message.error")} : ${res.message}`,
				});
			}
		})
		.catch((err) => {
			console.error(title, err);
			toast.error(title, {
				description: `${t("message.error")} : ${err.message}`,
			});
		});
	open.value = false;
};

watch(open, (newValue) => {
	if (newValue) {
		init();
	}
});
</script>
