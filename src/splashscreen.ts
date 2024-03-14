import { createApp } from "vue";

import Splashscreen from "./Splashscreen.vue";
import "./styles/splashscreen.css";
import { disableContextMenu, disableRefresh } from "./utils/Webview";

const splashscreenApp = createApp(Splashscreen);
splashscreenApp.mount("#splashscreen");

disableRefresh();
disableContextMenu();
