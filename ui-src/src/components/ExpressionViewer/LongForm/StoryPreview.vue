<template>
  <!-- story expression -->
  <div class="canvas__expression canvas__story">
    <!-- expression story -->
    <p class="canvas__story--title">{{ longFormData.title }}</p>
    <p class="canvas__story--body">
      {{ longFormData.body }}
    </p>
  </div>
</template>

<script>
export default {
  name: "LongForm",
  props: {
    longFormData: {
      title: String,
      body: String,
    }
  }
};
</script>
