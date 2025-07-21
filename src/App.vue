<template>
	<Suspense>
		<div class="grid grid-rows-[3.5rem_1fr_1.5rem] grid-cols-1 h-dvh overflow-hidden">
			<Header />
			<Home />
			<Footer />
		</div>
	</Suspense>
	<Toaster />
</template>

<script setup lang="ts">
import { Footer } from "@/components/footer";
import { Header } from "@/components/header";
import Home from "@/views/Home.vue";
import { Toaster } from "@/components/ui/sonner";
</script>
