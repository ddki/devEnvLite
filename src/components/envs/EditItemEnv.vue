<template>
	<Popover>
		<PopoverTrigger as-child>
			<slot />
		</PopoverTrigger>
		<PopoverContent class="w-84">
			<div class="grid gap-4">
				<div class="grid gap-2">
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="groupId">{{ t('envGroup.id') }}</Label>
						<Input v-model="data.groupId" type="text" :placeholder="t('envGroup.id')" class="col-span-2 h-8" readonly />
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="key">{{ t('env.key') }}</Label>
						<Input v-model="data.key" type="text" :placeholder="t('env.key')" class="col-span-2 h-8" />
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="value">{{ t('env.value') }}</Label>
						<Textarea v-model.trim="data.value" :placeholder="t('env.value')" class="col-span-2 h-8" />
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="note">{{ t('env.note') }}</Label>
						<Textarea v-model="data.note" :placeholder="t('env.note')" class="col-span-2 h-8" />
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
import { deleteEnv, getEnv, getGroupEnv, saveEnvToGroup } from "@/store";
import type { Env } from "@/store/type";
import { watch } from "vue";
import { onMounted, reactive } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

const { t } = useI18n();

interface Prop {
	configId: string;
	groupId: string;
	envKey?: string;
	maxSort?: number;
	operate: "edit" | "new";
}
const props = withDefaults(defineProps<Prop>(), {
	maxSort: 0,
});
console.log("props[item-env]: ", props);
const emit = defineEmits(["callback"]);

const data = reactive({
	configId: props.configId,
	groupId: props.groupId,
	key: props.envKey as string,
	value: "",
	note: "",
	sort: 0,
});

const onClear = () => {
	data.configId = props.configId;
	data.groupId = props.groupId;
	data.key = "";
	data.value = "";
	data.note = "";
	data.sort = props.maxSort + 1;
};

const onSave = async () => {
	const title =
		props.operate === "new"
			? `${t("operate.new")}${t("env.text")}`
			: `${t("operate.edit")}${t("env.text")}`;
	if (!props.configId || !data.groupId) {
		toast.warning(title, {
			description: t("env.error.selectGroup"),
		});
		return;
	}
	if (!data.key) {
		toast.warning(title, {
			description: t("env.error.keyNotEmpty"),
		});
		return;
	}
	if (
		props.operate === "new" &&
		(await checkGroupEnvsKeyExists(props.configId, data.groupId, data.key))
	) {
		toast.warning(title, {
			description: t("env.error.keyExists"),
		});
		return;
	}
	if (props.operate === "edit" && props.envKey) {
		await deleteEnv(props.configId, props.groupId, props.envKey);
	}
	const save = await saveEnvToGroup(props.configId, data);
	if (save) {
		emit("callback");
	} else {
		toast.error(title, {
			description: `${t("operate.save")}${t("message.failure")}`,
		});
	}
	if (props.operate === "new") {
		onClear();
	}
};

const checkGroupEnvsKeyExists = async (configId: string, groupId: string, envKey: string) => {
	if (props.envKey === envKey) {
		return false;
	}
	const result = await getGroupEnv(configId, groupId).then((group) => {
		if (group?.envs) {
			return group.envs.map((env: Env) => env.key).includes(envKey);
		}
		return false;
	});
	return result;
};

onMounted(async () => {
	if (props.operate === "edit" && props.configId && props.groupId && props.envKey) {
		await getEnv(props.configId, props.groupId, props.envKey).then((env) => {
			if (env) {
				data.groupId = env.groupId;
				data.key = env.key;
				data.value = env.value;
				data.note = env.note as string;
				data.sort = env.sort;
			}
		});
	} else {
		onClear();
	}
});

watch(props, async (newValue, _oldValue) => {
	if (newValue.operate === "edit" && newValue.configId && newValue.groupId && newValue.envKey) {
		await getEnv(newValue.configId, newValue.groupId, newValue.envKey).then((env) => {
			if (env) {
				data.groupId = env.groupId;
				data.key = env.key;
				data.value = env.value;
				data.note = env.note as string;
				data.sort = env.sort;
			}
		});
	} else {
		onClear();
	}
});
</script>
