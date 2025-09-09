<template>
	<Dialog>
		<DialogTrigger as-child>
			<slot />
		</DialogTrigger>
		<DialogContent>
			<DialogHeader>
				<DialogTitle>{{ title }}</DialogTitle>
				<DialogDescription>
				</DialogDescription>
				<DialogClose />
			</DialogHeader>
			<div class="grid gap-2">
				<div class="grid grid-cols-3 items-center gap-4">
					<Label for="key">{{ t('env.key') }}</Label>
					<Input v-model="data.key" type="text" :placeholder="t('env.key')" class="col-span-2 h-8" />
				</div>
				<div class="grid grid-cols-3 items-center gap-4">
					<Label for="value">{{ t('env.value') }}</Label>
					<Textarea v-model.trim="data.value" :placeholder="t('env.value')" class="col-span-2 h-8" />
				</div>
				<div class="grid grid-cols-3 items-center gap-4">
					<Label for="description">{{ t('env.description') }}</Label>
					<Textarea v-model="data.description" :placeholder="t('env.description')" class="col-span-2 h-8" />
				</div>
				<div class="grid grid-cols-3 items-center gap-4">
					<Label for="sort">{{ t('env.sort') }}</Label>
					<Input v-model="data.sort" type="number" :placeholder="t('env.sort')" class="col-span-2 h-8" />
				</div>
			</div>
			<DialogFooter>
				<Button variant="secondary" @click="onClear">
					{{ t("operate.clear") }}
				</Button>
				<Button @click="onSave">
					{{ t("operate.save") }}
				</Button>
			</DialogFooter>
		</DialogContent>
	</Dialog>
</template>

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
import { Textarea } from "@/components/ui/textarea";
import type { EnvironmentVariable, Res } from "@/types";
import { DefaultValue } from "@/types/defaultValue";
import { invoke } from "@tauri-apps/api/core";
import { computed, inject, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

const { t } = useI18n();

interface Prop {
	id?: string;
	groupId: string;
	maxSort?: number;
	operate: "edit" | "new";
}
const props = withDefaults(defineProps<Prop>(), {
	maxSort: 0,
});

const data = ref<EnvironmentVariable>({
	...DefaultValue.environmentVariable(),
	id: props.id,
});

const title = computed(() => {
	return props.operate === "new"
		? `${t("operate.new")}${t("env.text")}`
		: `${t("operate.edit")}${t("env.text")}`;
});

const reloadVariableGroupList: () => Promise<void> =
	inject("reloadVariableGroupList") || (async () => {});

const onClear = () => {
	data.value = {
		...DefaultValue.environmentVariable(),
		sort: props.maxSort + 1,
	};
};

// 检查变量名是否存在在配置中
const checkVariableKeyExists = async (): Promise<boolean> => {
	try {
		const res = await invoke<Res<boolean>>("check_variable_key_exists_in_config", {
			groupId: props.groupId,
			excludeVariableId: props.id,
			key: data.value.key,
		});
		if (res.code === "200") {
			if (res.data) {
				toast.warning(title, {
					description: t("env.error.keyExists"),
				});
				return true;
			}
			return res.data;
		}
		toast.error(title, {
			description: `${t("message.error")} : ${res.message}`,
		});
		return true;
	} catch (err) {
		toast.error(title, {
			description: `${t("message.error")} : ${err}`,
		});
		return true;
	}
};

// 创建环境变量
const create = async (title: string) => {
	await invoke<Res<boolean>>("create_environment_variable", {
		groupId: props.groupId,
		variable: {
			...DefaultValue.environmentVariable(),
			...data.value,
		},
	})
		.then(async (res) => {
			if (res.code === "200") {
				if (res.data) {
					toast.success(title, {
						description: t("message.success"),
					});
					await reloadVariableGroupList();
					return;
				}
			} else {
				toast.error(title, {
					description: `${t("message.error")} : ${res.message}`,
				});
				return;
			}
		})
		.catch((err) => {
			toast.error(title, {
				description: `${t("message.error")} : ${err.message}`,
			});
		})
		.finally(() => {
			onClear();
		});
};

// 更新环境变量
const update = async (title: string) => {
	await invoke<Res<boolean>>("update_environment_variable", {
		groupId: props.groupId,
		variable: {
			...DefaultValue.environmentVariable(),
			...data.value,
		},
	})
		.then(async (res) => {
			if (res.code === "200") {
				toast.success(title, {
					description: t("message.success"),
				});
				await reloadVariableGroupList();
			} else {
				toast.error(title, {
					description: `${t("message.error")} : ${res.message}`,
				});
			}
		})
		.catch((err) => {
			toast.error(title, {
				description: `${t("message.error")} : ${err.message}`,
			});
		});
};

const onSave = async () => {
	const title =
		props.operate === "new"
			? `${t("operate.new")}${t("env.text")}`
			: `${t("operate.edit")}${t("env.text")}`;
	if (!data.value.key) {
		toast.warning(title, {
			description: t("env.error.keyNotEmpty"),
		});
		return;
	}
	if (!data.value.value) {
		toast.warning(title, {
			description: t("env.error.valueNotEmpty"),
		});
		return;
	}
	// 检查变量名是否存在
	const keyExists = await checkVariableKeyExists();
	if (keyExists) {
		return;
	}

	// 更新环境变量
	if (props.operate === "edit" && props.id) {
		await update(title);
		return;
	}
	// 创建环境变量
	if (props.operate === "new") {
		await create(title);
		return;
	}
};

// 加载环境变量组
const loadVariable = async (id: string) => {
	const title = `${t("operate.query")}${t("env.text")}`;
	await invoke<Res<EnvironmentVariable>>("get_environment_variable", { id })
		.then((res) => {
			if (res.code === "200") {
				data.value = {
					...res.data,
				};
			} else {
				toast.error(title, {
					description: `${t("message.failure")}: ${res.message}`,
				});
			}
		})
		.catch((e) => {
			toast.error(title, {
				description: `${t("message.error")}: ${e.message}`,
			});
		});
};

onMounted(async () => {
	if (props.operate === "edit" && props.groupId && props.id) {
		await loadVariable(props.id);
	} else {
		onClear();
	}
});

watch(props, async (newValue, _oldValue) => {
	if (newValue.operate === "edit" && newValue.groupId && newValue.id) {
		await loadVariable(newValue.id);
	} else {
		onClear();
	}
});
</script>
