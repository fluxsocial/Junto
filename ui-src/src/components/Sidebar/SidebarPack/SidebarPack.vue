<template>
  <section class="sidebar">
    <p class="sidebar__header">PACKS</p>
    <p class="sidebar__border">&nbsp;</p>
    <div class="sidebar__packs">
      <div class="sidebar__pack">
        <p class="sidebar__pack--active">&nbsp;</p>
        <img
          :src="profile.entry.profile_picture"
          alt=""
          class="sidebar__pack--profile"
        />
        <!--These should be icons and not images - there are no images for packs in the holochain app currently-->
        <p class="sidebar__pack--name">{{ user_pack.entry.name }}</p>
      </div>

      <div
        v-for="item of joined_packs"
        :key="item.entry.name"
        class="sidebar__pack"
      >
        {{ item.entry.name }}>
        <img
          src="./../../../assets/img/junto-web__eric.png"
          alt=""
          class="sidebar__pack--profile"
        />
        <p class="sidebar__sphere--text">Mees Tomatoes</p>
      </div>
    </div>
  </section>
</template>

<script>
import packHttpMethods from "../../Pack/PackHttp";
import userHttpMethods from "../../User/UserHttp";
import Cookies from "js-cookie";

export default {
  name: "SidebarPack",
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
      user_pack: {
        address: null,
        entry: {
          name: null,
          owner: null,
          privacy: null
        }
      },
      joined_packs: [],
      cookieStore: Cookies.getJSON("cookieStore")
    };
  },
  mounted() {
    this.getPacks();
  },
  methods: {
    async hasUsernameAddress() {
      if (
        this.$store.getters.getUsername.address == null &&
        this.cookieStore == undefined
      ) {
        let result = await userHttpMethods.getUserProfileByAgentAddress(this);
        this.username = result.Ok.username;
        this.profile = result.Ok.profile;
      } else if (this.cookieStore != undefined) {
        const getUsernameCookie = this.cookieStore.userUsername;
        const getProfileCookie = this.cookieStore.userProfile;
        this.username = getUsernameCookie;
        this.profile = getProfileCookie;
      } else {
        this.username = this.$store.getters.getUsername;
        this.profile = this.$store.getters.getProfile;
      }
    },

    async getPacks() {
      await this.hasUsernameAddress();
      //Gets own pack of current user
      if (this.$store.getters.getPack.address == null) {
        packHttpMethods
          .getUsersPack(this, this.username.address)
          .then(result => {
            this.user_pack = result.Ok;
          });
      } else {
        this.user_pack = this.$store.getters.getPack;
      }

      //Here get the packs the user has joined
    }
  }
};
</script>
