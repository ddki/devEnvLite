import path from "node:path";
import { fileURLToPath } from "node:url";

import VueI18nPlugin from "@intlify/unplugin-vue-i18n/vite";
import tailwindcss from "@tailwindcss/vite";
import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
	plugins: [
		vue(),
		tailwindcss(),
		VueI18nPlugin({
			include: [path.resolve(__dirname, "./src/locales/*.yml")],
		}),
	],
	resolve: {
		alias: {
			"@": path.resolve(__dirname, "./src"),
		},
	},
	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	//
	// 1. prevent vite from obscuring rust errors
	clearScreen: false,
	// 2. tauri expects a fixed port, fail if that port is not available
	server: {
		port: 5173,
		strictPort: true,
		watch: {
			// 3. tell vite to ignore watching `src-tauri`
			ignored: ["**/src-tauri/**"],
		},
	},
	build: {
		target: "esnext",
		rollupOptions: {
			input: {
				index: fileURLToPath(new URL("./index.html", import.meta.url)),
			},
		},
	},
}));
