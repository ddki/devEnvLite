<script setup lang="ts">
import { cn } from "@/lib/utils";
import { reactiveOmit } from "@vueuse/core";
import { ProgressIndicator, ProgressRoot, type ProgressRootProps } from "reka-ui";
import type { HTMLAttributes } from "vue";

const props = withDefaults(defineProps<ProgressRootProps & { class?: HTMLAttributes["class"] }>(), {
	modelValue: 0,
});

const delegatedProps = reactiveOmit(props, "class");
</script>

<template>
  <ProgressRoot
    data-slot="progress"
    v-bind="delegatedProps"
    :class="
      cn(
        'bg-primary/20 relative h-2 w-full overflow-hidden rounded-full',
        props.class,
      )
    "
  >
    <ProgressIndicator
      data-slot="progress-indicator"
      class="bg-primary h-full w-full flex-1 transition-all"
      :style="`transform: translateX(-${100 - (props.modelValue ?? 0)}%);`"
    />
  </ProgressRoot>
</template>
