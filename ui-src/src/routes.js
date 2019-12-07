import Den from "./components/Den/Den.vue";
import Pack from "./components/Pack/Pack.vue";
import Collective from "./components/Collective/Collective.vue";
import LotusOpen from "./components/Lotus/LotusOpen/LotusOpen.vue";
import User from "./components/User/User";
import store from "./store.js";

const checkSourceChain = (to, from, next) => {
  //check if user has account in their source chain and redirect accordingly
  next('/register')
}

export const routes = [
  { path: "/", component: Den, beforeEnter: checkSourceChain },
  { path: "/user", component: User, beforeEnter: checkSourceChain },
  { path: "/pack", component: Pack, beforeEnter: checkSourceChain },
  { path: "/collective", component: Collective, beforeEnter: checkSourceChain },
  { path: "/lotus", component: LotusOpen, beforeEnter: checkSourceChain }
];
