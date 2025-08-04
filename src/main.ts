import messages from "@intlify/unplugin-vue-i18n/messages";
import { createApp } from "vue";
import { createI18n } from "vue-i18n";

import App from "./App.vue";
import router from "./router";
import "./styles/index.css";
import { invoke } from "@tauri-apps/api/core";
import mitt from "mitt";
import type { Res, Setting } from "./types";
import { disableContextMenu, disableRefresh } from "./utils/Webview";

const emitter = mitt();
emitter.on("reloadApp", () => {
	window.location.reload();
});

const setting = (await invoke<Res<Setting>>("get_settings").then((res) => {
	if (res.code === "200") {
		return res.data;
	}
})) || {
	language: "zh-CN",
	homeDir: "",
	cacheDir: "",
	dataDir: "",
	logDir: "",
	envBackupDir: "",
};

console.log("Current settings:", setting);

const i18n = createI18n({
	locale: setting.language || "zh-CN",
	messages,
});

const app = createApp(App);
app.config.globalProperties.$emitter = emitter;
app.use(i18n);
app.use(router);
app.mount("#main");

const setup = async () => {
	console.log("app setup...");
	disableRefresh(false);
	disableContextMenu(true);
};

await setup();
