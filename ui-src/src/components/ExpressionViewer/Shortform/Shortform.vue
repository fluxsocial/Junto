<template>
  <!-- shortform expression -->
  <div class="canvas__expression canvas__story">  
    <!-- expression shortform -->
    <p class="canvas__story--title">{{ shortFormData.text }}</p>
  </div>
</template>

<script>
export default {
  name: "Shortform",
  props: {
    shortFormData: {
      text: String
    }
  }
};
</script>
