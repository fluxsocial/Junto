<template>
  <section class="user">
    <JuntoNav class="navigation--den">
      <img
        slot="navigationLogo"
        class="navigation__top--logo"
        src="./../../assets/img/junto-web__logo--grey.png"
      />
      <div slot="navigationBorder" class="navigation__border"></div>
    </JuntoNav>
    <router-view />
  <div>
    {{ username.entry.username }}
    {{ profile.entry.first_name }}
    {{ profile.entry.last_name }}
    {{ profile.entry.bio }}
    {{ profile.entry.profile_picture }}
    {{ profile.entry.verified }}
  </div>
  </section>
</template>

<script>
import Nav from "./../Nav/Nav.vue";
import getUserProfile from "./UserHttp.js";

export default {
  name: "User",
  props: ["address"],
  components: {
    JuntoNav: Nav
  },
  data: function() {
    return {
      username: {
        address: String,
        entry: {
          username: String
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
    }
  },
  mounted() {
    this.userProfileOnInit();
  },
  methods: {
    userProfileOnInit() {
      if (this._props.address == "self" && this.$store.getters.getUsername == undefined){ //Check that we dont already have self data in store
        console.log("we do not have self data getting by agent address");
        target_user_profile = getUserProfileByAgentAddress(this);
        this.username = target_user_profile.username;
        this.profile = target_user_profile.profile;
      } else if (this._props.address != "self") { //Looking for data on some target user
        console.log("getting profile of some target user");
        target_user_profile = getUserProfileByUsernameAddress(this, this._props.address);
        this.username = target_user_profile.username;
        this.profile = target_user_profile.profile;
      } else { //The user data is already in the store
        console.log("we already have the profile data in store");
        this.username = this.$store.getters.getUsername;
        this.profile = this.$store.getters.getProfile;
      };
    }
  }
};
</script>
