import Vue from "vue";
import Vuex from "vuex";
import { connect } from "@holochain/hc-web-client";
import { Settings } from "./settings.js";

Vue.use(Vuex);

const privacy_types = {
  Private: "Private",
  Shared: "Shared",
  Public: "Public"
};

export const store = new Vuex.Store({
  state: {
    address: null,
    private_den: {
      address: null,
      entry: {
        parent: null,
        name: null,
        privacy: null,
        channel_type: null
      }
    },
    public_den: {
      address: null,
      entry: {
        parent: null,
        name: null,
        privacy: null,
        channel_type: null
      }
    },
    shared_den: {
      address: null,
      entry: {
        parent: null,
        name: null,
        privacy: null,
        channel_type: null
      }
    },
    pack: {
      address: null,
      entry: {
        name: null,
        owner: null,
        privacy: null
      }
    },
    profile: {
      address: null,
      entry: {
        parent: null,
        first_name: null,
        last_name: null,
        bio: null,
        profile_picture: null,
        verified: null
      }
    },
    username: {
      address: null,
      entry: {
        username: null
      }
    },
    user_perspective: {
      address: null,
      entry: {
        parent: null,
        name: null,
        privacy: null,
        channel_type: null
      }
    },
    nav_bar_location: Number,
    holochain_connection: connect({ url: Settings.Uri }) //Here connections is happening via settings - in the future when migration to holoscape occurs this will be blank and it can infer the url from holoscape
  },
  mutations: {
    //sync
    addUserHolochainData(state, data) {
      state.address = data.Ok.username.address;
      if (data.Ok.private_den != null) {
        state.private_den = data.Ok.private_den;
      };
      state.public_den = data.Ok.public_den;
      state.shared_den = data.Ok.shared_den;
      state.pack = data.Ok.pack;
      state.profile = data.Ok.profile;
      state.username = data.Ok.username;
      state.user_perspective = data.Ok.user_perspective;
    },
    addUserPackData(state, data) {
      state.pack = data.Ok;
    }
  },
  actions: {
    //async
  },
  getters: {
    getAddress: state => state.address,
    getPrivateDen: state => state.private_den,
    getPublicDen: state => state.public_den,
    getSharedDen: state => state.shared_den,
    getPack: state => state.pack,
    getProfile: state => state.profile,
    getUsername: state => state.username,
    getUserPerspective: state => state.user_perspective,
    getState: state => state,
    getHolochainConnection: state => state.holochain_connection
  }
});
