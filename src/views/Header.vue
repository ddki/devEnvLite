<template>
	<header class="flex flex-row justify-between pl-2 pr-2">
		<button type="button" class="btn btn-primary btn-sm sm:btn-sm md:btn-md" @click="onCollate">
			{{ t('header.collate') }}
		</button>
		<button type="button" class="btn btn-primary btn-sm sm:btn-sm md:btn-md" onclick="setting_modal.showModal()">
			{{ t('header.setting') }}
		</button>
	</header>
	<dialog id="setting_modal" class="modal modal-bottom sm:modal-middle">
		<div class="modal-box">
			<form method="dialog">
				<button class="btn btn-sm sm:btn-sm md:btn-md btn-circle btn-ghost absolute right-2 top-2">✕</button>
			</form>
			<h3 class="font-bold text-lg">{{ t('header.setting') }}</h3>
			<p class="py-4">
				<Suspense>
					<template #default>
						<Setting @callBack="postSaveSetting" />
					</template>
				</Suspense>
			</p>
		</div>
	</dialog>
</template>

<script setup lang="ts">
import { defineAsyncComponent, getCurrentInstance } from "vue";
import { useI18n } from "vue-i18n";
const global = getCurrentInstance()?.appContext.config.globalProperties;
const Setting = defineAsyncComponent(() => import("./Setting.vue"));

const { t } = useI18n();

const onCollate = () => {
	console.log(global?.$toast)
	global?.$toast.success("哈哈哈哈")
};

const postSaveSetting = () => {
	const settingModal = document.getElementById("setting_modal") as HTMLDialogElement
	settingModal.close()
}
</script>
