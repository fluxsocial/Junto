import Vue from "vue";
import Vuex from "vuex";
import { connect } from "@holochain/hc-web-client";
import { Settings } from "./settings.js";
import VuexPersistence from "vuex-persist";
import Cookies from "js-cookie";

Vue.use(Vuex);

const privacy_types = {
  Private: "Private",
  Shared: "Shared",
  Public: "Public"
};

const vuexLocalStorage = new VuexPersistence({
  key: "myLocalStore",
  storage: localStorage,
  reducer: state => ({
    userProfilePicture: state.user.profile.entry.profile_picture,
    userProfileBio: state.user.profile.entry.bio,
    base: state.base
  })
});

const vuexCookie = new VuexPersistence({
  key: "cookieStore",
  restoreState: (key, storage) => Cookies.getJSON(key),
  saveState: (key, state, storage) =>
    Cookies.set(key, state, {
      expires: 3,
      secure: false
    }),
  //Only include user module within cookies -- minus profile_picture and bio(character length exceeds limit)
  reducer: state => ({
    userAddress: state.user.address,
    userPrivateDen: state.user.private_den,
    userPublicDen: state.user.public_den,
    userSharedDen: state.user.shared_den,
    userPack: state.user.pack,
    userUsername: state.user.username,
    userUserPerspective: state.user.user_perspective,
    userProfile: {
      address: state.user.profile.address,
      entry: {
        first_name: state.user.profile.entry.first_name,
        last_name: state.user.profile.entry.last_name,
        verified: state.user.profile.entry.verified
      }
    },
    configDnaName: state.config.dna_name,
    configDnaAddress: state.config.dna_address,
    configAgentId: state.config.agent_id,
    configAgentAddress: state.config.agent_address,
    configCapRequest: state.config.cap_request
  })
  // filter: (mutation) => mutation.type == "addUserHolochainData"
});

const baseModule = {
  state: {
    nav_bar_location: Number,
    holochain_connection: connect({ url: Settings.Uri }) //Here connections is happening via settings - in the future when migration to holoscape occurs this will be blank and it can infer the url from holoscape
  },
  getters: {
    getState: state => state,
    getHolochainConnection: state => state.holochain_connection
  }
};

const userModule = {
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
    }
  },
  mutations: {
    //sync
    addUserHolochainData(state, data) {
      state.address = data.Ok.username.address;
      if (data.Ok.private_den != null) {
        state.private_den = data.Ok.private_den;
      }
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
    getUserPerspective: state => state.user_perspective
  }
};

const configModule = {
  state:{
    dna_name: null,
    dna_address: null,
    agent_id: {
      nick: null,
      pub_sign_key: null
    },
    agent_address: null,
    cap_request: {
      cap_token: null,
      provenance: null,
    }
  },
  mutations: {
    getConfigData(state, data) {
      state.dna_name = data.Ok.dna_name,
      state.dna_address = data.Ok.dna_address,
      state.agent_id = data.Ok.agent_id,
      state.agent_address = data.Ok.agent_address,
      state.cap_request = data.Ok.cap_request
    }
  },
  actions: {},
  getters: {
    getDnaName: state => state.dna_name,
    getDnaAddress: state => state.dna_address,
    getAgentId: state => state.agent_id,
    getAgentAddress: state => state.agent_address,
    getCapRequest: state => state.cap_request
  }
}

export const store = new Vuex.Store({
  modules: {
    base: baseModule,
    user: userModule,
    config: configModule
  },
  plugins: [vuexCookie.plugin, vuexLocalStorage.plugin]
});
