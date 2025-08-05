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
						<Label for="note">{{ t('envGroup.note') }}</Label>
						<Textarea v-model="data.note" :placeholder="t('envGroup.note')" class="col-span-2 h-8" />
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
import { deleteGroupEnv, getConfig, getGroupEnv, saveGroupEnvToConfig } from "@/store";
import type { GroupEnv } from "@/store/type";
import { v4 as uuidv4 } from "uuid";
import { watch } from "vue";
import { onMounted, reactive } from "vue";
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
const emit = defineEmits(["callback"]);

const data = reactive({
	id: "",
	configId: props.configId,
	name: "",
	note: "",
	sort: 0,
});

const onClear = () => {
	data.id = uuidv4();
	data.configId = props.configId;
	data.name = "";
	data.note = "";
	data.sort = props.maxSort + 1;
};

const onSave = async () => {
	const title =
		props.operate === "new"
			? t("operate.new", { name: t("envGroup.text") })
			: t("operate.edit", { name: t("envGroup.text") });
	if (!data.configId) {
		toast.warning(title, {
			description: t("envGroup.error.selectConfig"),
		});
		return;
	}
	if (!data.name) {
		toast.warning(title, {
			description: t("envGroup.error.nameNotEmpty"),
		});
		return;
	}
	if (props.operate === "new" && (await checkGroupEnvNameExists(props.configId, data.name))) {
		toast.warning(title, {
			description: t("envGroup.error.nameExists"),
		});
		return;
	}
	if (props.operate === "edit" && props.id) {
		await deleteGroupEnv(props.configId, props.id);
	}
	const save = await saveGroupEnvToConfig(data);
	if (save) {
		emit("callback");
	} else {
		toast.error(title, {
			description: t("message.operate-failure", { operate: t("operate.save") }),
		});
	}
	if (props.operate === "new") {
		onClear();
	}
};

const checkGroupEnvNameExists = async (configId: string, groupEnvName: string) => {
	const result = await getConfig(configId).then((config) => {
		if (config.groupEnvs) {
			return config.groupEnvs.map((group: GroupEnv) => group.name).includes(groupEnvName);
		}
		return false;
	});
	return result;
};

onMounted(async () => {
	if (props.operate === "edit" && props.configId && props.id) {
		const store = await getGroupEnv(props.configId, props.id);
		data.id = store.id;
		data.configId = store.configId;
		data.name = store.name;
		data.note = store.note as string;
		data.sort = store.sort;
	} else {
		onClear();
	}
});

watch(props, async (newValue, _oldValue) => {
	if (newValue.operate === "edit" && newValue.configId && newValue.id) {
		const store = await getGroupEnv(newValue.configId, newValue.id);
		data.id = store.id;
		data.configId = store.configId;
		data.name = store.name;
		data.note = store.note as string;
		data.sort = store.sort;
	} else {
		onClear();
	}
});
</script>
