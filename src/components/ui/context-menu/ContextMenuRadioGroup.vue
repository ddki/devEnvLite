<script setup lang="ts">
import {
	ContextMenuRadioGroup,
	type ContextMenuRadioGroupEmits,
	type ContextMenuRadioGroupProps,
	useForwardPropsEmits,
} from "reka-ui";

const props = defineProps<ContextMenuRadioGroupProps>();
const emits = defineEmits<ContextMenuRadioGroupEmits>();

const forwarded = useForwardPropsEmits(props, emits);
</script>

<template>
  <ContextMenuRadioGroup
    data-slot="context-menu-radio-group"
    v-bind="forwarded"
  >
    <slot />
  </ContextMenuRadioGroup>
</template>
