import { createApp } from "vue";
import Toast from "./index.vue";
const createToast = (options: { type: string; message: string; duration?: number }) => {
	const toastInstance = createApp(Toast, {
		type: options.type,
		message: options.message,
		duration: options.duration
	});

	const el = document.createElement("div");
	toastInstance.mount(el);

	document.body.appendChild(el);

	let timer = 0
  clearTimeout(timer)
	timer = setTimeout(() => {
		toastInstance.unmount();
		document.body.removeChild(el);
	}, options.duration || 2000);
};

const toast = {
	show: (type: string, message: string, duration?: number) =>
		createToast({ type, message, duration }),
	info: (message: string, duration?: number) => createToast({ type: "info", message, duration }),
	success: (message: string, duration?: number) =>
		createToast({ type: "success", message, duration }),
	warning: (message: string, duration?: number) =>
		createToast({ type: "warning", message, duration }),
	error: (message: string, duration?: number) => createToast({ type: "error", message, duration }),
};

export default {
	// biome-ignore lint/suspicious/noExplicitAny: <explanation>
	install: (app: any) => {
		app.config.globalProperties.$toast = toast;
		app.provide("toast", toast);
	},
};
