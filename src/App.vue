<template>
	<Suspense>
		<template #default>
			<el-config-provider :locale="locale">
				<Header />
				<Main />
			</el-config-provider>
		</template>
	</Suspense>
</template>


<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ElConfigProvider } from "element-plus";
import { ref } from "vue";
import Header from "./views/Header.vue";
import Main from "./views/MainContent.vue";

import en from "element-plus/dist/locale/en.mjs";
import zhCn from "element-plus/dist/locale/zh-cn.mjs";
import { getSetting } from "./store/setting";

const locale = ref(en);

getSetting().then((res) => {
	if (res.language === "zh") {
		locale.value = zhCn;
	}
});

invoke("close_splashscreen");
</script>
