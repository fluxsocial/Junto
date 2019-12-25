<template>
  <section class="user">
    <JuntoNav class="navigation--den">
      <img
        slot="navigationLogo"
        class="navigation__top--logo"
        src="./../../assets/img/junto-web__logo--grey.png"
      />
      <div
        slot="navigationBorder"
        class="navigation__border"
      ></div>
    </JuntoNav>
    <router-view />
  </section>
</template>

<script>
import Nav from "./../Nav/Nav.vue";

export default {
  name: "User",
  components: {
    JuntoNav: Nav
  }
};
</script>
