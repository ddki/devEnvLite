<template>
	<Suspense>
		<template #default>
			<el-config-provider :locale="locale">
				<div class="common-layout">
					<el-container>
						<el-header>
							<Header />
						</el-header>
						<el-main>
							<Main />
						</el-main>
						<el-footer>
							<Footer />
						</el-footer>
					</el-container>
				</div>
			</el-config-provider>
		</template>
	</Suspense>
</template>


<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { defineAsyncComponent, ref } from "vue";
import { ElConfigProvider } from 'element-plus'
import Header from "./views/Header.vue";
const Main from "./views/MainContent.vue";
const Footer from "./views/Footer.vue";

import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import en from "element-plus/dist/locale/en.mjs";
import { getSetting } from "./store/setting";

const locale = ref(en);

getSetting()
	.then((res) => {
		if (res.language === 'zh') {
			locale.value = zhCn
		}
	});



invoke("close_splashscreen");
</script>
