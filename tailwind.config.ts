import type { Config } from "tailwindcss";
import plugin from "tailwindcss/plugin";

export default {
	content: ["./index.html", "./src/**/*.{vue,jsx,tsx}"],
	theme: {
		extend: {},
	},
	plugins: [
		plugin(({ addComponents }) => {
			const boxShadow = {
				".shadow-box": {
					boxShadow: "rgba(0, 0, 0, 0.35) 0px 5px 15px"
				}
			}
			addComponents(boxShadow);
		}),
		require("@tailwindcss/typography")
	]
} satisfies Config;
