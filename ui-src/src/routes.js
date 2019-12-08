import Pack from "./components/Pack/Pack.vue";
import Collective from "./components/Collective/Collective.vue";
import LotusOpen from "./components/Lotus/LotusOpen/LotusOpen.vue";
import User from "./components/User/User";
import Register from "./components/User/Register/Register";
import Account from "./components/User/Account/Account";
import Den from "./components/User/Den/Den.vue";
import PublicDen from "./components/User/Den/Public/PublicDen";
import PrivateDen from "./components/User/Den/Private/PrivateDen";
import NotFound from "./components/NotFound/404.vue";

import store from "./store.js";

const checkSourceChain = (to, from, next) => {
  //check if user has account in their source chain and redirect accordingly
  next(next);
};

export const routes = [
  { path: "/", component: Collective, beforeEnter: checkSourceChain },
  {
    path: "/user",
    component: User,
    beforeEnter: checkSourceChain,
    children: [
      {
        path: "register",
        component: Register
      },
      {
        path: "account",
        component: Account
      },
      {
        path: "den",
        component: Den,
        children: [
          {
            path: "public",
            component: PublicDen
          },
          {
            path: "private",
            component: PrivateDen
          }
        ]
      }
    ]
  },
  { path: "/pack", component: Pack, beforeEnter: checkSourceChain },
  { path: "/collective", component: Collective, beforeEnter: checkSourceChain },
  { path: "/lotus", component: LotusOpen, beforeEnter: checkSourceChain },
  { path: "*", component: NotFound }
];
