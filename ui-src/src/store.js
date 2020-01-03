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
    address: String,
    private_den: {
      address: String,
      entry: {
        parent: String,
        name: String,
        privacy: String,
        channel_type: String
      }
    },
    public_den: {
      address: String,
      entry: {
        parent: String,
        name: String,
        privacy: String,
        channel_type: String
      }
    },
    shared_den: {
      address: String,
      entry: {
        parent: String,
        name: String,
        privacy: String,
        channel_type: String
      }
    },
    pack: {
      address: String,
      entry: {
        name: String,
        owner: String,
        privacy: String
      }
    },
    profile: {
      address: String,
      entry: {
        parent: String,
        first_name: String,
        last_name: String,
        bio: String,
        profile_picture: String,
        verified: Boolean
      }
    },
    username: {
      address: String,
      entry: {
        username: String
      }
    },
    user_perspective: {
      address: String,
      entry: {
        parent: String,
        name: String,
        privacy: String,
        channel_type: String
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
    addProfileHolochainData(state, data) {
      state.profile = data.Ok;
    },
    addUsernameHolochainData(state, data) {
      state.username = data.Ok;
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
