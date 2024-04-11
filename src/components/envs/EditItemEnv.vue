<template>
	<Popover>
		<PopoverTrigger as-child>
			<slot />
		</PopoverTrigger>
		<PopoverContent>
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
						<Input v-model.trim="data.value" type="text" :placeholder="t('env.value')" class="col-span-2 h-8" />
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="note">{{ t('env.note') }}</Label>
						<Input v-model="data.note" type="text" :placeholder="t('env.note')" class="col-span-2 h-8" />
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="sort">{{ t('env.sort') }}</Label>
						<Input v-model="data.sort" type="text" :placeholder="t('env.sort')" class="col-span-2 h-8" />
					</div>
				</div>
				<div class="grid grid-cols-2 gap-4">
					<Button variant="secondary" @click="onClear">
						{{ t("clear") }}
					</Button>
					<Button @click="onSave">
						{{ t("save") }}
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
import { useToast } from "@/components/ui/toast/use-toast";
import { deleteEnv, getEnv, getGroupEnv, saveEnvToGroup } from "@/store";
import type { Env } from "@/store/type";
import { watch } from "vue";
import { onMounted, reactive } from "vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const { toast } = useToast();

interface Prop {
	configId: string,
	groupId: string,
	key?: string;
	maxSort?: number;
	operate: "edit" | "new";
}
const props = withDefaults(defineProps<Prop>(), {
	maxSort: 0,
});
console.log("props[item-env]: ", props);
const emit = defineEmits(["callback"]);

const data = reactive({
	groupId: props.groupId,
	key: props.key as string,
	value: "",
	note: "",
	sort: 0,
});

const onClear = () => {
	data.groupId = props.groupId;
	data.key = "";
	data.value = "";
	data.note = "";
	data.sort = props.maxSort + 1;
};

const onSave = async () => {
	if (!props.configId || !data.groupId) {
		toast({
			description: t("env.error.selectGroup"),
			variant: "destructive",
		});
		return;
	}
	if (!data.key) {
		toast({
			description: t("env.error.keyNotEmpty"),
			variant: "destructive",
		});
		return;
	}
	if (
		props.operate === "new" &&
		(await checkGroupEnvsKeyExists(props.configId, data.groupId, data.key))
	) {
		toast({
			description: t("env.error.keyExists"),
			variant: "destructive",
		});
		return;
	}
	if (props.operate === "edit" && props.key) {
		await deleteEnv(props.configId, props.groupId, props.key);
	}
	const save = await saveEnvToGroup(props.configId, data);
	if (save) {
		emit("callback");
	} else {
		toast({
			description: t("save") + t("failure"),
			variant: "destructive",
		});
	}
};

const checkGroupEnvsKeyExists = async (configId: string, groupId: string, envKey: string) => {
	if (props.key === envKey) {
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
	if (props.operate === "edit" && props.configId && props.groupId && props.key) {
		await getEnv(props.configId, props.groupId, props.key).then((env) => {
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
	if (newValue.operate === "edit" && newValue.configId && newValue.groupId && newValue.key) {
		await getEnv(newValue.configId, newValue.groupId, newValue.key).then((env) => {
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
})
</script>
