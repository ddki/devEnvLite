<script setup lang="ts">
import { ScrollArea } from "@/components/ui/scroll-area";
import {
	Select,
	SelectContent,
	SelectGroup,
	SelectItem,
	SelectItemText,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select";
import { cn } from "@/lib/utils";
import { setTheme } from "@tauri-apps/api/app";
import { set, useColorMode } from "@vueuse/core";
import { Moon, Sun, SunMoon } from "lucide-vue-next";
import { watch } from "vue";

interface Prop {
	placeholder?: string;
}

const props = defineProps<Prop>();

const mode = useColorMode({
	emitAuto: true,
	modes: {
		rose_light: "rose_light",
		rose_dark: "rose_dark",
		green_light: "green_light",
		green_dark: "green_dark",
		blue_light: "blue_light",
		blue_dark: "blue_dark",
		orange_light: "orange_light",
		orange_dark: "orange_dark",
	},
});

const themes = [
	"auto",
	"light",
	"dark",
	"rose_light",
	"rose_dark",
	"green_light",
	"green_dark",
	"blue_light",
	"blue_dark",
	"orange_light",
	"orange_dark",
];

watch(mode, (newValue) => {
	console.log("theme changed: ", newValue);
	if (newValue.includes("dark")) {
		setTheme("dark");
	} else if (newValue.includes("light")) {
		setTheme("light");
	} else {
		setTheme(null);
	}
});
</script>

<template>
	<Select v-model="mode">
		<SelectTrigger>
			<SelectValue :placeholder="props.placeholder">
				<div class="grid grid-flow-col gap-1">
					<SunMoon v-if="mode === 'auto'" class="w-5 h-5" />
					<Moon v-else-if="mode === 'dark'" class="w-5 h-5" />
					<Sun v-else-if="mode === 'light'" class="w-5 h-5" />
					<div v-else class="w-5 h-5 bg-primary" />
					<span>{{ mode }}</span>
				</div>
			</SelectValue>
		</SelectTrigger>
		<SelectContent class="max-h-[90dvh]">
			<ScrollArea>
				<SelectGroup>
					<SelectItem v-for="item in themes" :key="item" :value="item" :text-value="item">
						<SelectItemText>
							<div :class="cn('grid grid-flow-col gap-4 items-center justify-between', item)">
								<div class="w-32 grid grid-flow-col gap-2 items-center justify-start">
									<SunMoon v-if="item === 'auto'" class="w-5 h-5" />
									<Moon v-else-if="item === 'dark'" class="w-5 h-5" />
									<Sun v-else-if="item === 'light'" class="w-5 h-5" />
									<div v-else class="w-5 h-5 bg-primary" />
									<span>{{ item }}</span>
								</div>
								<div class="grid grid-cols-4 border card">
									<div class="grid grid-rows-2">
										<div class="h-6 w-12 bg-primary-foreground" />
										<div class="h-6 w-12 bg-primary" />
									</div>
									<div class="grid grid-rows-2">
										<div class="h-6 w-12 bg-secondary-foreground" />
										<div class="h-6 w-12 bg-secondary" />
									</div>
									<div class="grid grid-rows-2">
										<div class="h-6 w-12 bg-muted-foreground" />
										<div class="h-6 w-12 bg-muted" />
									</div>
									<div class="grid grid-rows-2">
										<div class="h-6 w-12 bg-destructive-foreground" />
										<div class="h-6 w-12 bg-destructive" />
									</div>
								</div>
							</div>
						</SelectItemText>
					</SelectItem>
				</SelectGroup>
			</ScrollArea>
		</SelectContent>
	</Select>
</template>
