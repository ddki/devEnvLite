<template>
	<Popover>
		<PopoverTrigger as-child>
			<slot />
		</PopoverTrigger>
		<PopoverContent>
			<div class="grid gap-4">
				<div class="grid gap-2">
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="id">{{ t('config.id') }}</Label>
						<Input v-model="data.id" type="text" :placeholder="t('config.id')" class="col-span-2 h-8" readonly />
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="name">{{ t('config.name') }}</Label>
						<Input v-model.trim="data.name" type="text" :placeholder="t('config.name')" class="col-span-2 h-8" />
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="note">{{ t('config.note') }}</Label>
						<Input v-model="data.note" type="text" :placeholder="t('config.note')" class="col-span-2 h-8" />
					</div>
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="sort">{{ t('config.sort') }}</Label>
						<Input v-model="data.sort" type="text" :placeholder="t('config.sort')" class="col-span-2 h-8" />
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
import { deleteConfig, getConfig, getConfigNames, saveConfig } from "@/store";
import { v4 as uuidv4 } from "uuid";
import { onMounted, reactive } from "vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const { toast } = useToast();

interface Prop {
	id?: string;
	maxSort?: number;
	operate: "edit" | "new";
}
const props = withDefaults(defineProps<Prop>(), {
	maxSort: 0,
});

const emit = defineEmits(["callback"]);

const data = reactive({
	id: "",
	name: "",
	note: "",
	sort: 0,
});

const onClear = () => {
	data.id = uuidv4();
	data.name = "";
	data.note = "";
	data.sort = props.maxSort + 1;
};

const onSave = async () => {
	if (!data.name) {
		toast({
			description: t("config.error.nameNotEmpty"),
			variant: "destructive",
		});
		return;
	}
	const configNames = await getConfigNames();
	console.log("configNames = ", configNames);
	if (props.operate === "new" && configNames?.includes(data.name)) {
		toast({
			description: t("config.error.nameExists"),
			variant: "destructive",
		});
		return;
	}
	if (props.operate === "edit" && props.id) {
		await deleteConfig(props.id);
	}
	const save = await saveConfig(data);
	if (save) {
		emit("callback");
	} else {
		toast({
			description: t("save") + t("failure"),
			variant: "destructive",
		});
	}
};

onMounted(async () => {
	if (props.operate === "edit" && props.id) {
		const storeConfig = await getConfig(props.id);
		data.id = storeConfig.id;
		data.name = storeConfig.name;
		data.note = storeConfig.note as string;
		data.sort = storeConfig.sort;
	} else {
		onClear();
	}
});
</script>
