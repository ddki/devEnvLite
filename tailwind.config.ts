import type { Config } from "tailwindcss";
import plugin from "tailwindcss/plugin";

export default {
	content: ["./index.html", "./src/**/*.{vue,jsx,tsx}"],
	theme: {
		extend: {
			colors: {
				primary: "#0099CC",
				second: "#666666",
			},
			gridTemplateColumns: {
				main: "20dvw minmax(60dvw, 1fr) 20dvw",
			},
		},
	},
	plugins: [
		plugin(({ addComponents }) => {
			const button = {
				".button": {
					backgroundColor: "#0099CC",
					padding: ".2rem 1rem",
					color: "#fff",
					borderRadius: ".2rem",
					"&:hover": {
						backgroundColor: "#666666",
					},
				},
			};
			addComponents(button);

			const tags = {
				".tag": {
					backgroundColor: "#666666",
					padding: ".2rem .4rem",
					color: "#fff",
					borderRadius: ".5rem",
				},
			};
			addComponents(tags);

			const link = {
				".link": {
					color: "#0099CC",
					"&:hover": {
						color: "#666666",
					},
				},
			};
			addComponents(link);
		}),
	],
} satisfies Config;
