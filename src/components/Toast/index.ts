import { createApp } from "vue";
import Toast from "./index.vue";
const createToast = (options: {
	type: string;
	message: string;
	duration?: number;
	position?: string[];
}) => {
	const toastInstance = createApp(Toast, {
		type: options.type,
		message: options.message,
		duration: options.duration,
		position: options.position,
	});

	const el = document.createElement("div");
	toastInstance.mount(el);

	document.body.appendChild(el);

	let timer = 0;
	clearTimeout(timer);
	timer = setTimeout(() => {
		toastInstance.unmount();
		document.body.removeChild(el);
	}, options.duration || 2000);
};

const toast = {
	show: (type: string, message: string, position?: string[], duration?: number) =>
		createToast({ type, message, duration, position }),
	info: (message: string, position?: string[], duration?: number) =>
		createToast({ type: "info", message, duration, position }),
	success: (message: string, position?: string[], duration?: number) =>
		createToast({ type: "success", message, duration, position }),
	warning: (message: string, position?: string[], duration?: number) =>
		createToast({ type: "warning", message, duration, position }),
	error: (message: string, position?: string[], duration?: number) =>
		createToast({ type: "error", message, duration, position }),
};

export default {
	// biome-ignore lint/suspicious/noExplicitAny: <explanation>
	install: (app: any) => {
		app.config.globalProperties.$toast = toast;
		app.provide("toast", toast);
	},
};
