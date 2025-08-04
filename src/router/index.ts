import { type RouteRecordRaw, createMemoryHistory, createRouter } from "vue-router";

const routes: RouteRecordRaw[] = [
	{
		path: "/",
		name: "Home",
		component: () => import("@/views/Home.vue"),
	},
];

const router = createRouter({
	history: createMemoryHistory(import.meta.env.BASE_URL),
	routes,
});

export default router;
