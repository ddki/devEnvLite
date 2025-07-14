import messages from "@intlify/unplugin-vue-i18n/messages";
import { createApp } from "vue";
import { createI18n } from "vue-i18n";

import { getSetting } from "@/store/setting";
import App from "./App.vue";
import "./styles/index.css";
import mitt from "mitt";
import { disableContextMenu, disableRefresh } from "./utils/Webview";

const emitter = mitt();
emitter.on("reloadApp", () => {
	window.location.reload();
});

const setting = await getSetting();

const i18n = createI18n({
	locale: setting.language || "en",
	messages,
});

const app = createApp(App);
app.config.globalProperties.$emitter = emitter;
app.use(i18n);
app.mount("#main");

// disableRefresh();
disableContextMenu();
