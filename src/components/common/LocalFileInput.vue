<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";
import { type DialogFilter, open } from "@tauri-apps/plugin-dialog";
import { File, Folder, FolderOpenDot } from "lucide-vue-next";
import { type HTMLAttributes, defineModel } from "vue";

interface Props {
	type: "file" | "folder";
	placeholder?: string;
	accepts?: string[];
	class?: HTMLAttributes["class"];
}
const props = withDefaults(defineProps<Props>(), { type: "file", accepts: () => ["json"] });
const model = defineModel({
	type: String,
});

const getFilePath = async () => {
	const filters: DialogFilter[] = [];
	if (props.accepts && props.accepts.length > 0) {
		filters.push({
			name: "default",
			extensions: props.accepts,
		});
	}
	const file = await open({
		multiple: false,
		directory: props.type === "folder",
		filters: filters,
		defaultPath: model.value,
	});
	console.log(file);
	if (file) {
		model.value = props.type === "folder" ? (file as unknown as string) : file?.path;
	}
};
</script>

<template>
	<div :class="cn('relative items-center', props.class)">
		<Input v-model="model" type="text" :placeholder="props.placeholder" />
		<Button variant="ghost" size="icon" @click="getFilePath"
			class="absolute end-0 inset-y-0 flex items-center justify-center px-2">
			<File class="size-6 text-muted-foreground" v-if="props.type === 'file'" />
			<Folder class="size-6 text-muted-foreground" v-else />
		</Button>
	</div>
</template>
