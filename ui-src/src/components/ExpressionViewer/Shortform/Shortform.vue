<template>
  <!-- shortform expression -->
  <div class="canvas__expression canvas__story">
    <!-- expression top -->
    <div class="canvas__expression--top">
      <div class="canvas__expression--profile">
        <button class="canvas__expression--profile--picture">&nbsp;</button>

        <div class="canvas__expression--profile--details">
          <p class="canvas__expression--profile--name">
            {{ shortFormData.users_name }}
          </p>
          <p class="canvas__expression--profile--handle">
            {{ shortFormData.username }}
          </p>
        </div>
      </div>
    </div>

    <!-- expression shortform -->
    <p class="canvas__story--title">{{ shortFormData.text }}</p>

    <!-- expression bottom -->
    <div class="canvas__expression--bottom">
      <div class="canvas__expression--channels">
        <button class="canvas__expression--channel">design</button>
        <button class="canvas__expression--channel">philosophy</button>
      </div>

      <div class="canvas__expression--responses">
        <button class="canvas__expression--resonate">&nbsp;</button>
        <button class="canvas__expression--comment">&nbsp;</button>
      </div>
    </div>
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
