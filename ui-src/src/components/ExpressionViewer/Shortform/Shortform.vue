<template>
  <!-- shortform expression -->
  <div class="canvas__expression canvas__story">  
    <!-- expression shortform -->
    <p class="canvas__story--title">{{ shortFormData.text }}</p>
  </div>
</template>

<script>
import expressionViewerHttpMethods from "./../ExpressionViewerHttp";

export default {
  name: "Shortform",
  props: {
    shortFormData: {
      users_name: String,
      username: String,
      text: String
    }
  }
};
</script>
