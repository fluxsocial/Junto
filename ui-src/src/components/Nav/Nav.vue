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

        <router-link to="/" class="navigation__top--logo">
          <slot name="navigationLogo"></slot>
        </router-link>

        <Dropdown
          name="notifs-nav-dropdown"
          class="navigation__top--notifs"
          :Items="notifItems"
          Icon="icon-moon"
        />

        <Dropdown
          name="den-nav-dropdown"
          class="navigation__top--den"
          :Items="accountItems"
          Icon="icon-account"
        />
      </div>
      <slot name="navigationBorder"></slot>
      <slot name="navigationBottom"></slot>
    </div>
  </div>
</template>

<script>
import Dropdown from "../Dropdown/Dropdown";

export default {
  components: {
    Dropdown
  },
  data() {
    return {
      accountItems: [
        {
          text: "Profile",
          url: "/user/self"
        },
        {
          text: "Public Den",
          url: "/user/den/public"
        },
        {
          text: "Private Den",
          url: "/user/den/private"
        },
        {
          text: "Log Out",
          url: "/user/logout"
        }
      ],
      notifItems: [
        //write method to get user notifications and push all to this array
        {
          text: "notif 1",
          url: "/user/notifications"
        },
        {
          text: "notif 2",
          url: "/user/notifications"
        },
        {
          text: "notif 3",
          url: "/user/notifications"
        }
      ]
    };
  }
};
</script>
