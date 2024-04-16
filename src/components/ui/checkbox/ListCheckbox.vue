<script setup lang="ts">
import { type HTMLAttributes, ref, onBeforeMount } from "vue";
import { cn } from "@/lib/utils";
import { Checkbox } from ".";


interface Props {
	items: { label: string; value: string; }[];
}

const props = defineProps<Props & { class?: HTMLAttributes["class"] }>();
const emits = defineEmits(["update:modelValue"]);
const model = defineModel<string[]>();

const checkValues = ref<string[]>([]);
const checkList = ref<{ label: string; value: string; checked: boolean }[]>();

onBeforeMount(() => {
	checkList.value = props.items.map((item) => ({ ...item, checked: false }));
})

const updateChecked = (item: { label: string; value: string; checked: boolean }) => {
	item.checked =!item.checked;
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
</script>

<template>
	<div :class="cn('grid gap-1', props.class)">
		<div class="flex items-center space-x-2" v-for="item in checkList">
			<Checkbox :id="item.value" :name="item.value" :checked="item.checked"
				@update:checked="updateChecked(item)" />
			<label for="terms"
				class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
				{{ item.label }}
			</label>
		</div>
	</div>
</template>
