import { createApp } from "vue";

import App from "./App.vue";
import { createI18nInstance } from "./locales";
import "./styles/index.css";
import mitt from "mitt";
import { disableContextMenu, disableRefresh } from "./utils/Webview";

const emitter = mitt();
emitter.on("reloadApp", () => {
	window.location.reload();
});

const app = createApp(App);
app.config.globalProperties.$emitter = emitter;
app.use(await createI18nInstance());
app.mount("#main");

const setup = async () => {
	const env = import.meta.env.MODE || "development";
	const disable = env === "production";
	console.log("app setup... ", import.meta.env.MODE, env);
	// 禁止刷新
	disableRefresh(disable);
	// 禁止右键菜单
	disableContextMenu(disable);
};

await setup();
