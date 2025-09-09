import type { Res, Setting } from "@/types";
import messages from "@intlify/unplugin-vue-i18n/messages";
import { invoke } from "@tauri-apps/api/core";
import { createI18n } from "vue-i18n";
import { isTauri } from "../utils/env";

// 动态初始化 i18n
export async function createI18nInstance() {
	let locale = "zh-CN";

	if (isTauri()) {
		try {
			// 仅在 Tauri 环境下调用后端接口获取配置
			const setting = await invoke<Res<Setting>>("get_settings").then((res) => {
				if (res.code === "200") {
					return res.data;
				}
			});
			console.log("i18n setting", setting);
			locale = setting?.language || "zh-CN";
		} catch (e) {
			console.warn("Tauri 环境下获取 i18n 配置失败，使用默认配置", e);
		}
	}

	console.log("i18n locale", locale);

	return createI18n({
		legacy: false, // Vue 3 推荐使用 composition API
		locale,
		messages,
	});
}
