import { createApp } from "vue";
import "./styles/tailwind.css";
import "@imengyu/vue3-context-menu/lib/vue3-context-menu.css";
import ContextMenu from "@imengyu/vue3-context-menu";
import { createI18n } from "vue-i18n";
import messages from "@intlify/unplugin-vue-i18n/messages";

const i18n = createI18n({
	locale: "en",
	messages,
});

import App from "./App.vue";
import './styles/index.css'

const app = createApp(App);
app.use(i18n);
app.use(ContextMenu);
app.mount("#app");

// document.addEventListener("keydown", (event) => {
// 	if (
// 		event.key === "F5" ||
// 		(event.ctrlKey && event.key.toLowerCase() === "r") ||
// 		(event.metaKey && event.key.toLowerCase() === "r")
// 	) {
// 		event.preventDefault();
// 	}
// 	if (
// 		event.key === "F12" ||
// 		(event.ctrlKey && event.shiftKey && event.key.toLowerCase() === "i") ||
// 		(event.metaKey && event.shiftKey && event.key.toLowerCase() === "i")
// 	) {
// 		event.preventDefault();
// 	}
// });
// document.addEventListener("contextmenu", (event) => {
// 	event.preventDefault();
// });
