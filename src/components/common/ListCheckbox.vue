<script setup lang="ts">
import { cn } from "@/lib/utils";
import { type HTMLAttributes, ref, watch } from "vue";
import { Checkbox } from "@/components/ui/checkbox";

interface CheckInfo {
	label: string;
	value: string;
	checked: boolean;
}
interface Props {
	items: CheckInfo[];
}

const props = defineProps<Props & { class?: HTMLAttributes["class"] }>();
const emits = defineEmits(["update:modelValue"]);
const model = defineModel<string[]>();
const options = ref<CheckInfo[]>(props.items);

const checkValues = ref<string[]>([]);

const updateChecked = (item: CheckInfo) => {
	item.checked = !item.checked;
	if (item.checked) {
		if (checkValues.value.includes(item.value)) {
			return;
		}
		checkValues.value.push(item.value);
	} else {
		checkValues.value = checkValues.value.filter((v) => v !== item.value);
	}
	model.value = checkValues.value;
	console.log("item, checkValues, model ", item, checkValues.value, model.value);
};

watch(
	() => props.items,
	(newItems) => {
		options.value = newItems;
	},
	{
		deep: true,
	},
);
</script>

<template>
	<div :class="cn('grid gap-1', props.class)">
		<div class="flex items-center space-x-2" v-for="item in options">
			<Checkbox :id="item.value" :name="item.value" :checked="item.checked" @update:checked="updateChecked(item)" />
			<label for="terms"
				class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
				{{ item.label }}
			</label>
		</div>
	</div>
</template>
