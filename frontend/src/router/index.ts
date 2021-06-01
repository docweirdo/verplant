import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import HomeCard from "../views/HomeCard.vue";
import Page404 from "../views/404.vue";
import store from "@/store";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: HomeCard,
    beforeEnter: (to, from, next) => {
      store.bookingUrl.value = null;
      next();
    },
  },
  {
    path: "/my-booking/:bookingUrl",
    name: "Booking",
    component: HomeCard,
    beforeEnter: (to, from, next) => {
      if (typeof to.params.bookingUrl === "string") {
        store.bookingUrl.value = to.params.bookingUrl;
      }
      next();
    },
  },
  {
    path: "/login",
    name: "Login",
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () =>
      import(/* webpackChunkName: "about" */ "../views/Login.vue"),
  },
  {
    path: "/:pathMatch(.*)*", // see https://next.router.vuejs.org/guide/migration/#removed-star-or-catch-all-routes why * doesn't work anymore
    name: "404",
    component: Page404,
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

(window as any).router = router;

export default router;
