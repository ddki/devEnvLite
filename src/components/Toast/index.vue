<template>
	<div :class="toastClass">
		<div :class="alertClass">
			<span>{{ props.message }}</span>
		</div>
	</div>
</template>

<script setup lang="ts">
const props = defineProps({
	type: String,
	message: String,
	duration: {
		type: Number,
		default: 2000,
	},
	position: {
		type: Array,
		default: ["end", "bottom"],
	},
});
console.log("toast props = ", props);
/**
 * toast-start 	align horizontally to the left
	toast-center 	align horizontally to the center
	toast-end 	align horizontally to the right (default)
	toast-top 	align vertically to top
	toast-middle 	align vertically to middle
	toast-bottom 	align vertically to bottom (default)
 */
const [x, y] = props.position;
const toastClass = `toast toast-${x} toast-${y} z-[9999]`;
const alertClass = `alert alert-${props.type}`;
</script>
