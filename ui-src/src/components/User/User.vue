<template>
  <section class="user">
    <JuntoNav>
      <img
        slot="navigationLogo"
        class="navigation__top--logo"
        src="./../../assets/img/junto-web__logo--grey.png"
      />
      <div
        slot="navigationBorder"
        class="navigation__border navigation__border--pack"
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
    JuntoNav: Nav,
  }
};
</script>
