<template>
  <!-- expression bottom -->
  <div class="canvas__expression--bottom">
    <div class="canvas__expression--channels">
      <!-- channels would be looped and rendered here -->
      <button
        v-for="channel in channels"
        :key="channel.entry.value"
        class="canvas__expression--channel"
      >
        {{ channel.entry.value }}
      </button>
    </div>

    <div class="canvas__expression--responses">
      <button class="canvas__expression--comment">Comment</button>
    </div>
  </div>
</template>

<script>
export default {
  name: "ExpressionBottom",
  props: {
    channels: Array
  }
};
</script>
>
