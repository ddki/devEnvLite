<template>
	<Popover>
		<PopoverTrigger as-child>
			<slot />
		</PopoverTrigger>
		<PopoverContent class="w-84">
			<div class="grid gap-4">
				<div class="text-lg font-bold text-center bg-secondary p-2 rounded-md">{{ title }}</div>
				<div class="grid gap-2">
					<!-- <div class="grid grid-cols-3 items-center gap-4">
						<Label for="id">{{ t('config.id') }}</Label>
						<Input v-model="data.id" type="text" :placeholder="t('config.id')" class="col-span-2 h-8" readonly />
					</div> -->
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="name" class="text-right">{{ t('config.import-config.types.env.scope') }}</Label>
						<RadioGroup v-model="data.scope">
							<div class="flex items-center space-x-2" v-for="item in scopesList" :key="item.value">
								<RadioGroupItem :value="item.value" />
								<Label for="r1">{{ item.label }}</Label>
							</div>
						</RadioGroup>
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="name">{{ t('config.name') }}</Label>
						<Input v-model.trim="data.name" type="text" :placeholder="t('config.name')" class="col-span-2 h-8" />
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="description">{{ t('config.description') }}</Label>
						<Textarea v-model="data.description" :placeholder="t('config.description')" class="col-span-2 h-8" />
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="sort">{{ t('config.sort') }}</Label>
						<Input v-model="data.sort" type="number" :placeholder="t('config.sort')" class="col-span-2 h-8" />
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
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Textarea } from "@/components/ui/textarea";
import { getEnvironmentVariableScopeList } from "@/constants";
import type { EnvConfig, Res } from "@/types";
import { DefaultValue } from "@/types/defaultValue";
import { checkConfigNameExists } from "@/utils/ValidateUtil";
import { invoke } from "@tauri-apps/api/core";
import { type Ref, computed, inject, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

const { t } = useI18n();
const scopesList = getEnvironmentVariableScopeList(t);

interface Prop {
	id?: string;
	operate: "edit" | "new";
}
const props = defineProps<Prop>();

const emit = defineEmits(["reload"]);

const data = ref<EnvConfig>({
	...DefaultValue.envConfig(),
});

const title = computed(() => {
	return props.operate === "new"
		? `${t("operate.new")}${t("config.text")}`
		: `${t("operate.edit")}${t("config.text")}`;
});

const envConfigs = inject<Ref<EnvConfig[]>>("envConfigs");

const currentSort = computed(() => {
	return envConfigs?.value?.length
		? Math.max(...envConfigs.value.map((item) => item.sort ?? 0)) + 1
		: 1;
});

const onClear = () => {
	data.value = {
		...DefaultValue.envConfig(),
		sort: currentSort.value,
	};
};

// 创建环境变量配置
const create = async (title: string) => {
	await invoke<Res<void>>("create_env_config", {
		config: {
			...DefaultValue.envConfig(),
			...data.value,
		},
	})
		.then((res) => {
			if (res.code === "200") {
				emit("reload");
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
};

// 更新环境变量配置
const update = async (title: string) => {
	await invoke<Res<void>>("update_env_config", {
		config: {
			...DefaultValue.envConfig(),
			...data.value,
		},
	})
		.then((res) => {
			if (res.code === "200") {
				toast.success(title, {
					description: t("message.success"),
				});
				emit("reload");
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

const onSave = async () => {
	const title =
		props.operate === "new"
			? `${t("operate.new")}${t("config.text")}`
			: `${t("operate.edit")}${t("config.text")}`;
	if (!data.value.scope) {
		toast.warning(title, {
			description: t("config.error.scopesNotEmpty"),
		});
		return;
	}
	if (!data.value.name) {
		toast.warning(title, {
			description: t("config.error.nameNotEmpty"),
		});
		return;
	}

	if (checkConfigNameExists(envConfigs?.value || [], data.value.id, data.value.name)) {
		toast.warning(title, {
			description: t("config.error.nameExists"),
		});
		return;
	}

	// 编辑
	if (props.operate === "edit" && props.id) {
		await update(title);
		return;
	}

	// 新增
	if (props.operate === "new") {
		await create(title);
		return;
	}
};

onMounted(async () => {
	if (props.operate === "edit" && props.id) {
		await invoke<Res<EnvConfig>>("get_env_config", { id: props.id })
			.then((res) => {
				if (res.code === "200") {
					const storeConfig = res.data;
					data.value = {
						...storeConfig,
					};
				}
			})
			.catch((e) => {
				toast.error(`${t("operate.query")}${t("config.text")}`, {
					description: `${t("message.error")}: ${e.message}`,
				});
				return;
			});
	} else {
		onClear();
	}
});
</script>
