import ContextMenu from "@imengyu/vue3-context-menu";
import "@imengyu/vue3-context-menu/lib/vue3-context-menu.css";
import messages from "@intlify/unplugin-vue-i18n/messages";
import { createApp } from "vue";
import { createI18n } from "vue-i18n";
import "./styles/tailwind.css";

import App from "./App.vue";
import { getSetting } from "./store/setting";
import "./styles/main.css";
import { disableContextMenu, disableRefresh } from "./utils/Webview";
import Toast from "./components/Toast/index";

const setting = await getSetting();

console.log(setting)

const i18n = createI18n({
	locale: setting.language || "en",
	messages,
});

const app = createApp(App);
app.use(i18n);
app.use(ContextMenu);
app.use(Toast);
app.mount("#main");

// 设置daisyui主题
document.documentElement.setAttribute("data-theme", setting.theme || "light");
// disableRefresh();
// disableContextMenu();
