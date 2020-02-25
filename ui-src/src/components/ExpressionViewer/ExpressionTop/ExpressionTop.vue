<template>
  <div class="canvas__expression--top">
    <div class="canvas__expression--profile" @click="routeToProfile()">
      <button class="canvas__expression--profile--picture">&nbsp;</button>
      <div class="canvas__expression--profile--details">
        <p class="canvas__expression--profile--name">
          {{ users_first_name }} {{ users_last_name }}
        </p>
        <p class="canvas__expression--profile--handle">
          {{ username }}
        </p>
      </div>
    </div>
  </div>
</template>

<script>
import Cookies from "js-cookie";

export default {
  name: "ExpressionTop",
  props: {
    users_first_name: String,
    users_last_name: String,
    username: String,
    user_address: String
  },
  methods: {
    routeToProfile() {
      if (
        this.user_address == Cookies.getJSON("cookieStore").userUsername.address
      ) {
        this.$router.push("/user/self");
      } else {
        this.$router.push({
          path: "user/" + this.user_address
        });
      }
    }
  }
};
</script>
