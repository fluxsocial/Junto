import Den from "./components/Den/Den.vue";
import Pack from "./components/Pack/Pack.vue";
import Collective from "./components/Collective/Collective.vue";
import LotusOpen from "./components/Lotus/LotusOpen/LotusOpen.vue";
import User from "./components/User/User";
import { makeHolochainCall, isSuccess } from "./utils.js";
import { connect } from "@holochain/hc-web-client";
import { Settings } from "./settings.js";
import store from "./store.js";

const checkSourceChain = async (to, from, next) => {
  //check if user has account in their source chain and redirect accordingly
  let connection = connect( { url: Settings.Uri } );
  makeHolochainCall(connection, "user", "get_user_profile_by_agent_address", {}, (result) => {
      console.log(result);
      if (isSuccess(result) == true) {
          console.log("User has registed here it is: ", result);
          next(next)
      } else {
          next("/register")
      };
  });
}

export const routes = [
  { path: "/", component: Den, beforeEnter: checkSourceChain },
  { path: "/user", component: User, beforeEnter: checkSourceChain },
  { path: "/pack", component: Pack, beforeEnter: checkSourceChain },
  { path: "/collective", component: Collective, beforeEnter: checkSourceChain },
  { path: "/lotus", component: LotusOpen, beforeEnter: checkSourceChain }
];
