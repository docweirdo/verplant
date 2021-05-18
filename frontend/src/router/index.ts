import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import HomeCard from "../views/HomeCard.vue";
import Page404 from "../views/404.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: HomeCard,
  },
  {
    path: "/my-booking/:bookingUrl", // can be accessed with $route.params.bookingUrl
    name: "Home",
    component: HomeCard,
    beforeEnter: (to, from, next) => {
      console.log(to.params.bookingUrl)
    }
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
    path: '/:pathMatch(.*)*', // see https://next.router.vuejs.org/guide/migration/#removed-star-or-catch-all-routes why * doesn't work anymore
    name: '404', 
    component: Page404 
  }
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
