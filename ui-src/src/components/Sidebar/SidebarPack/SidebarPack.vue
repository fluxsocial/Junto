<template>
  <section class="sidebar">
    <p class="sidebar__header">PACKS</p>
    <p class="sidebar__border">&nbsp;</p>
    <div class="sidebar__packs">
      <div class="sidebar__pack">
        <p class="sidebar__pack--active">&nbsp;</p>
        <img
          src="./../../../assets/img/junto-web__eric.png" 
          alt=""
          class="sidebar__pack--profile"
        /> <!--These should be icons and not images - there are no images for packs in the holochain app currently-->
        <p class="sidebar__pack--name">{{ user_pack.entry.name }}</p>
      </div>
      
      <div class="sidebar__pack" v-for="item of joined_packs"> {{ item.entry.name }}>
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

export default {
  name: "SidebarPack",
  data: function() {
    return {
      current_users_username_address: null,
      user_pack: {
        address: null,
        entry: {
          name: null,
          owner: null,
          privacy: null
        }
      },
      joined_packs: []
    }
  },
  mounted() {
    this.getPacks();
  },
  methods: {
    async hasUsernameAddress() {
      if (this.$store.getters.getUsername.address == null) {
        let result = await userHttpMethods.getUserProfileByAgentAddress(this);
        this.current_users_username_address = result.Ok.username.address;
      } else {
        this.current_users_username_address = this.$store.getters.getUsername.address;
      };
    },

    async getPacks() {
      await this.hasUsernameAddress();
      //Gets own pack of current user
      if (this.$store.getters.getPack.address == null) {
        packHttpMethods.getUsersPack(this, this.current_users_username_address).then(result => {
          this.user_pack = result.Ok;
        });
      } else {
        this.user_pack = this.$store.getters.getPack;
      };

      //Here get the packs the user has joined
    },
  }
};
</script>