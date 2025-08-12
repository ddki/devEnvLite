<template>
	<Popover>
		<PopoverTrigger as-child>
			<slot />
		</PopoverTrigger>
		<PopoverContent class="w-84">
			<div class="grid gap-4">
				<div class="text-lg font-bold text-center bg-secondary p-2 rounded-md">{{ title }}</div>
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
				<div class="grid grid-cols-2 gap-4">
					<Button variant="secondary" @click="onClear">
						{{ t("operate.clear") }}
					</Button>
					<Button @click="onSave">
						{{ t("operate.save") }}
					</Button>
				</div>
			</div>
		</PopoverContent>
	</Popover>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";
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

const onClear = () => {
	data.value = {
		...DefaultValue.environmentVariable(),
		sort: props.maxSort + 1,
	};
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

	// 检查变量名是否存在在配置中
	await invoke<Res<boolean>>("check_variable_key_exists_in_config", {
		config_id: await inject("configId"),
		exclude_variable_id: props.id,
		key: data.value.key,
	})
		.then((res) => {
			if (res.code === "200") {
				if (res.data) {
					toast.warning(title, {
						description: t("env.error.keyExists"),
					});
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
			return;
		});

	// 更新环境变量
	if (props.operate === "edit" && props.id) {
		await invoke<Res<boolean>>("update_environment_variable", {
			group_id: props.groupId,
			variable: data.value,
		})
			.then((res) => {
				if (res.code === "200") {
					if (res.data) {
						toast.success(title, {
							description: t("message.success"),
						});
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
				return;
			})
			.finally(() => {
				onClear();
			});
	}

	// 创建环境变量
	await invoke<Res<boolean>>("create_environment_variable", {
		group_id: props.groupId,
		variable: data.value,
	})
		.then((res) => {
			if (res.code === "200") {
				if (res.data) {
					toast.success(title, {
						description: t("message.success"),
					});
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
			return;
		})
		.finally(() => {
			onClear();
		});
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
