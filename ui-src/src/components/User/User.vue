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
    </div>
  </section>
</template>

<script>
import Nav from "./../Nav/Nav.vue";
import userHttpMethods from "./UserHttp.js";
import Cookies from "js-cookie";

export default {
  name: "User",
  components: {
    JuntoNav: Nav
  },
  props: ["address"],
  data() {
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
      }
    };
  },
  created() {
    console.log(
      "Within created() lifecycle hook, here's cookies: ",
      Cookies.getJSON('cookieStore')    //JSON.parse method throws error when cookies is empty
    );
  },
  mounted() {
    this.userProfileOnInit();
  },
  methods: {
    userProfileOnInit() {
      if (
        this._props.address == "self" &&
        this.$store.getters.getUsername.address == null &&
        Cookies.get("cookieStore") == undefined
      ) {
        //Check that we dont already have self data in store
        console.log("we do not have self data getting by agent address");
        userHttpMethods.getUserProfileByAgentAddress(this).then(result => {
          this.username = result.Ok.username;
          this.profile = result.Ok.profile;
        });
      } else if (this._props.address != "self") {
        //Looking for data on some target user
        console.log("getting profile of some target user");
        userHttpMethods
          .getUserProfileByUsernameAddress(this, this._props.address)
          .then(result => {
            this.username = result.Ok.username;
            this.profile = result.Ok.profile;
          });
      } else {
        //The user data is already in the store or cookies
        console.log("we already have the profile data in store or cookies");
        if (this.$store.getters.getUsername.address === null) {
					//Get majority of user profile from cookie storage
          const cookieStore = Cookies.getJSON("cookieStore");
          const getUsernameCookie = cookieStore.userUsername;
          const getProfileCookie = cookieStore.userProfile;
          this.username = getUsernameCookie;
          this.profile = getProfileCookie;

          //Get profile_picture and bio from window.localStorage
          if (localStorage.getItem("myLocalStore") !== null) {
            const browserStorage = JSON.parse(localStorage.getItem("myLocalStore"));
            this.profile.entry.profile_picture =
						browserStorage.userProfilePicture;
            this.profile.entry.bio = browserStorage.userProfileBio;
          }
        } else {
          this.username = this.$store.getters.getUsername;
          this.profile = this.$store.getters.getProfile;
        }
      }
    }
  }
};
</script>
