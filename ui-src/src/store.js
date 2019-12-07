import Vue from 'vue';
import Vuex from 'vuex';

Vue.use(Vuex)

const privacy_types = {
    Private: "Private",
    Shared: "Shared",
    Public: "Public",
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
                channel_type: String,
            },
        },
        public_den: {
            address: String,
            entry: {
                parent: String,
                name: String,
                privacy: String,
                channel_type: String,
            },
        },
        shared_den: {
            address: String,
            entry: {
                parent: String,
                name: String,
                privacy: String,
                channel_type: String,
            },
        },
        pack: {
            address: String,
            entry: {
                name: String,
                owner: String,
                privacy: String,
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
                verified: Boolean,
            }
        },
        username: {
            address: String,
            entry: {
                username: String,
            },
        },
        user_perspective: {
            address: String,
            entry: {
                parent: String,
                name: String,
                privacy: String,
                channel_type: String,
            },
        },
        nav_bar_location: Number,
	},
	mutations: { //syncronous
	},
	actions: { //async
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
    }
})
	