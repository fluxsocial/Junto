<template>
  <div class="lotus">
    <div class="lotus__content">
      <span class="lotus__icon" @click="initEnv">
        <slot name="lotusIcon"></slot>
      </span>
    </div>
  </div>
</template>

<script>
import lotusHttpMethods from "./LotusHttp";

export default {
  name: "Lotus",
  methods: {
    initEnv() {
      console.log("this here: ", this);
      lotusHttpMethods.getEnv(this);
      this.$router.push({name: "lotus"});
    }
  }
}
</script>