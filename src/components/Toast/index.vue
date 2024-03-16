<template>
	<div class="toast toast-end">
		<div class="alert `alert-{{ props.type }}`">
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
		default: 3000
	}
})
</script>
