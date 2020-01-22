import Pack from "./components/Pack/Pack.vue";
import Collective from "./components/Collective/Collective.vue";
import LotusOpen from "./components/Lotus/LotusOpen/LotusOpen.vue";
import User from "./components/User/User.vue";
import Register from "./components/User/Register/Register.vue";
import Den from "./components/User/Den/Den.vue";
import PublicDen from "./components/User/Den/Public/PublicDen.vue";
import PrivateDen from "./components/User/Den/Private/PrivateDen.vue";
import Profile from "./components/User/Profile/Profile.vue";
import NotFound from "./components/NotFound/404.vue";

import { makeHolochainCall, isSuccess } from "./utils.js";
import { connect } from "@holochain/hc-web-client";
import { Settings } from "./settings.js";
import store from "./store.js";

const checkSourceChain = async (to, from, next) => {
  //check if user has account in their source chain and redirect accordingly
  let connection = connect({ url: Settings.Uri });
  if (to.path != "/user/register") {
    makeHolochainCall(
      connection,
      "user",
      "get_user_profile_by_agent_address",
      {},
      result => {
        if (isSuccess(result) == true) {
          console.log(
            "(checkSourceChain) User is already registered here is their metadata: ",
            result
          );
          next(next);
        } else {
          next("/register");
        }
      }
    );
  } else {
    next();
  }
};

export const routes = [
  { path: "/", component: Collective, beforeEnter: checkSourceChain, name="home" },
  {
    path: "/user/:address",
    component: User,
    beforeEnter: checkSourceChain,
    props: true,
    children: [
      {
        path: "den",
        component: Den,
        children: [
          {
            path: "public",
            component: PublicDen,
            name: "publicDen"
          },
          {
            path: "private",
            component: PrivateDen,
            name: "privateDen"
          }
        ]
      }
    ]
  },
  {
    path: "/register",
    component: Register
  },
  { path: "/pack", component: Pack, beforeEnter: checkSourceChain, name:"pack" },
  { path: "/collective", component: Collective, beforeEnter: checkSourceChain, name="collective" },
  { path: "/lotus", component: LotusOpen, beforeEnter: checkSourceChain, name="lotus" },
  { path: "*", component: NotFound, name="notFound" }
];
