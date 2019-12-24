<template>
  <div class="navigation">
    <div class="navigation__content">
      <div class="navigation__top">
        <router-link to="/collective" class="navigation__top--collective">
          <svg class="navigation__top--icon">
            <use
              xlink:href="../../../src/assets/img/sprite.svg#icon-square"
            ></use>
          </svg>
        </router-link>

        <router-link to="/pack" class="navigation__top--pack">
          <svg class="navigation__top--icon">
            <use
              xlink:href="../../../src/assets/img/sprite.svg#icon-triangle"
            ></use>
          </svg>
        </router-link>
        <!-- <router-link to="/collective" class="navigation__top--collective">
          <svg class="navigation__top--icon">
            <use
              xlink:href="../../../src/assets/img/sprite.svg#icon-circle"
            ></use>
          </svg>
        </router-link> -->

        <router-link to="/" class="navigation__top--logo">
          <slot name="navigationLogo"></slot>
        </router-link>

        <div class="navigation__top--den" >
          <svg class="navigation__top--icon" >
            <use xlink:href="../../../src/assets/img/sprite.svg#icon-account"></use>
          </svg>
          <div class="navigation__top--den--dropdown" >
            <ul class="dropdown__list">
              <router-link tag="li" v-for="item in accountItems" :key="item.text" class="dropdown__list--item" :to="item.url">
                  {{ item.text }}
              </router-link>
            </ul>
          </div>
        </div>

        <router-link to="/user/notifications" class="navigation__top--notifs">
          <svg class="navigation__top--icon">
            <use xlink:href="../../../src/assets/img/sprite.svg#icon-moon"></use>
          </svg>
        </router-link>

      </div>
      <slot name="navigationBorder"></slot>
      <slot name="navigationBottom"></slot>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      accountItems: [
        {
          text: 'Profile',
          url: '/user'
        },
        {
          text: 'Public Den',
          url: '/user/den/public'
        },
        {
          text: 'Private Den',
          url: '/user/den/private'
        },
        {
          text: 'Log Out',
          url: '/user/logout'
        }
      ],
      notifItems: [],
    };
  }
};
</script>
