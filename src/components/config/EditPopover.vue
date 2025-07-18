<template>
	<Popover>
		<PopoverTrigger as-child>
			<slot />
		</PopoverTrigger>
		<PopoverContent class="w-84">
			<div class="grid gap-4">
				<div class="grid gap-2">
					<div class="grid grid-cols-3 items-center gap-4">
						<Label for="id">{{ t('config.id') }}</Label>
						<Input v-model="data.id" type="text" :placeholder="t('config.id')" class="col-span-2 h-8" readonly />
					</div>
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
						<Label for="note">{{ t('config.note') }}</Label>
						<Textarea v-model="data.note" :placeholder="t('config.note')" class="col-span-2 h-8" />
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
import { deleteConfig, getActiveConfigNames, getConfig, saveConfig } from "@/store";
import { v4 as uuidv4 } from "uuid";
import { onMounted, reactive } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";

const { t } = useI18n();

const scopesList = [
	{ label: t("env.scopes.user"), value: "USER" },
	{ label: t("env.scopes.system"), value: "SYSTEM" },
];

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
	scope: "USER",
	name: "",
	note: "",
	sort: 0,
});

const onClear = () => {
	data.id = uuidv4();
	data.scope = "USER";
	data.name = "";
	data.note = "";
	data.sort = props.maxSort + 1;
};

const onSave = async () => {
	if (!data.scope) {
		toast({
			title:
				props.operate === "new"
					? t("operate.new", { name: t("config.text") })
					: t("operate.edit", { name: t("config.text") }),
			description: t("config.error.scopesNotEmpty"),
			variant: "destructive",
		});
		return;
	}
	if (!data.name) {
		toast({
			title:
				props.operate === "new"
					? t("operate.new", { name: t("config.text") })
					: t("operate.edit", { name: t("config.text") }),
			description: t("config.error.nameNotEmpty"),
			variant: "destructive",
		});
		return;
	}
	const configNames = await getActiveConfigNames();
	console.log("configNames = ", configNames);
	if (props.operate === "new" && configNames?.includes(data.name)) {
		toast({
			title:
				props.operate === "new"
					? t("operate.new", { name: t("config.text") })
					: t("operate.edit", { name: t("config.text") }),
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
			title:
				props.operate === "new"
					? t("operate.new", { name: t("config.text") })
					: t("operate.edit", { name: t("config.text") }),
			description: t("message.operate-failure", { operate: t("operate.save") }),
			variant: "destructive",
		});
	}
	if (props.operate === "new") {
		onClear();
	}
};

onMounted(async () => {
	if (props.operate === "edit" && props.id) {
		const storeConfig = await getConfig(props.id);
		data.id = storeConfig.id;
		data.scope = storeConfig.scope;
		data.name = storeConfig.name;
		data.note = storeConfig.note as string;
		data.sort = storeConfig.sort;
	} else {
		onClear();
	}
});
</script>
