<template>
	<Popover>
		<PopoverTrigger as-child>
			<slot />
		</PopoverTrigger>
		<PopoverContent class="w-84">
			<div class="grid gap-4">
				<div class="grid gap-2">
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="configId">{{ t('config.id') }}</Label>
						<Input v-model="data.configId" type="text" :placeholder="t('config.id')" class="col-span-2 h-8" readonly />
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="id">{{ t('envGroup.id') }}</Label>
						<Input v-model="data.id" type="text" :placeholder="t('envGroup.id')" class="col-span-2 h-8" readonly />
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="name">{{ t('envGroup.name') }}</Label>
						<Input v-model.trim="data.name" type="text" :placeholder="t('envGroup.name')" class="col-span-2 h-8" />
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="description">{{ t('envGroup.description') }}</Label>
						<Textarea v-model="data.description" :placeholder="t('envGroup.description')" class="col-span-2 h-8" />
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="sort">{{ t('envGroup.sort') }}</Label>
						<Input v-model="data.sort" type="number" :placeholder="t('envGroup.sort')" class="col-span-2 h-8" />
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
import type { Res, VariableGroup } from "@/types";
import { DefaultValue } from "@/types/defaultValue";
import { invoke } from "@tauri-apps/api/core";
import { inject, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

const { t } = useI18n();

interface Prop {
	configId: string;
	id?: string;
	maxSort?: number;
	operate: "edit" | "new";
}
const props = withDefaults(defineProps<Prop>(), {
	maxSort: 0,
});
console.log("props[group-env]: ", props);

const data = ref<VariableGroup>({
	...DefaultValue.variableGroup(),
	id: props.id,
	configId: props.configId,
	name: "",
});

const onClear = () => {
	data.value = {
		...DefaultValue.variableGroup(),
		configId: props.configId,
		sort: props.maxSort + 1,
	};
};

const onSave = async () => {
	const title =
		props.operate === "new"
			? `${t("operate.new")}${t("envGroup.text")}`
			: `${t("operate.edit")}${t("envGroup.text")}`;
	if (!data.value.configId) {
		toast.warning(title, {
			description: t("envGroup.error.selectConfig"),
		});
		return;
	}
	if (!data.value.name) {
		toast.warning(title, {
			description: t("envGroup.error.nameNotEmpty"),
		});
		return;
	}
	if (checkVariableGroupNameExists(data.value.id, data.value.name)) {
		toast.warning(title, {
			description: t("envGroup.error.nameExists"),
		});
		return;
	}

	// 更新环境变量组
	if (props.operate === "edit" && props.id) {
		await invoke<Res<void>>("update_variable_group", { group: data.value })
			.then(async (res) => {
				if (res.code === "200") {
					await inject("reloadVariableGroupList");
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
	}
	// 创建环境变量组
	if (props.operate === "new") {
		await invoke<Res<string>>("create_variable_group", { group: data.value })
			.then(async (res) => {
				if (res.code === "200") {
					await inject("reloadVariableGroupList");
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
			})
			.finally(() => {
				onClear();
			});
	}
};

// 检查环境变量组名称是否存在
const checkVariableGroupNameExists = (excludeId: string | undefined, newName: string) => {
	return inject<VariableGroup[]>("variableGroupList")
		?.filter((item) => item.id !== excludeId)
		.map((item) => item.name)
		.includes(newName);
};
// 加载环境变量组
const loadVariableGroup = async (id: string) => {
	const title = `${t("operate.query")}${t("envGroup.text")}`;
	await invoke<Res<VariableGroup>>("get_variable_group", { groupId: id })
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
	if (props.operate === "edit" && props.configId && props.id) {
		await loadVariableGroup(props.id);
	} else {
		onClear();
	}
});

watch(props, async (newValue, _oldValue) => {
	if (newValue.operate === "edit" && newValue.configId && newValue.id) {
		await loadVariableGroup(newValue.id);
	} else {
		onClear();
	}
});
</script>
