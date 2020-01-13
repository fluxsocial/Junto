<template>
  <div class="nav-dropdown-container">
    <svg class="navigation__top--icon">
      <use :xlink:href="'../../../src/assets/img/sprite.svg#' + icon"></use>
    </svg>
    <div class="dropdown">
      <ul class="dropdown__list">
        <router-link
          v-for="item in items"
          :key="item.text"
          tag="li"
          class="dropdown__list--item"
          :to="item.url"
        >
          {{ item.text }}
        </router-link>
      </ul>
    </div>
  </div>
</template>

<script>
export default {
  name: "Dropdown",
  props: {
    items: {
      type: Array,
      required: true
    },
    icon: {
      type: String,
      required: true
    }
  },
  data() {
    return {};
  }
};
</script>
