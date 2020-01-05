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
import userHttpMethods from "./UserHttp.js";

export default {
  name: "User",
  props: ["address"],
  components: {
    JuntoNav: Nav
  },
  data: function() {
    return {
      username: {
        address: null,
        entry: {
          username: null
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
    }
  },
  mounted() {
    this.userProfileOnInit();
  },
  methods: {
    userProfileOnInit() {
      if (this._props.address == "self" && this.$store.getters.getUsername.address == null){ //Check that we dont already have self data in store
        console.log("we do not have self data getting by agent address");
        userHttpMethods.getUserProfileByAgentAddress(this).then(result => { //These two closures do not work
          this.username = result.Ok.username;
          this.profile = result.Ok.profile;
        });
      } else if (this._props.address != "self") { //Looking for data on some target user
        console.log("getting profile of some target user");
        userHttpMethods.getUserProfileByUsernameAddress(this, this._props.address).then(result => {
          this.username = result.Ok.username;
          this.profile = result.Ok.profile;
        });
      } else { //The user data is already in the store
        console.log("we already have the profile data in store", this.$store.getters.getUsername, this.$store.getters.getProfile);
        this.username = this.$store.getters.getUsername;
        this.profile = this.$store.getters.getProfile;
      };
    }
  }
};
</script>
