import { createMemoryHistory, createRouter, type RouteRecordRaw } from "vue-router";

const routes: RouteRecordRaw[] = [
	{
		path: "/splashscreen",
		name: "Splashscreen",
		component: () => import("@/views/Splashscreen.vue"),
	},
	{
		path: "/",
		name: "Home",
		component: () => import("@/views/Home.vue"),
	}
];

const router = createRouter({
	history: createMemoryHistory(import.meta.env.BASE_URL),
	routes,
});

export default router;
