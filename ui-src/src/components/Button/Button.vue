<template>
  <div :class="activeClass">
    <button class="btn btn--style-override" @click="buttonMethod()">
      {{ text }}
    </button>
  </div>
</template>

<script>
export default {
  name: "Button",
  props: {
    url: String, //Optional parameter -> URL link
    text: String, //Inner text of button
    activeClass: String, //class assignment to button container
    method: Function //method to be called on click
  },
  methods: {
    buttonMethod() {
      if (this.url) {
        this.method(this.url);
      } else {
        this.method();
      }
    }
  }
};
</script>
