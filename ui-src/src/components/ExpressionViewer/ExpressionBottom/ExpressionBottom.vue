<template>
    <!-- expression bottom -->
    <div class="canvas__expression--bottom">
      <div class="canvas__expression--channels">
        <!-- channels would be looped and rendered here -->
        <button class="canvas__expression--channel">design</button>
        <button class="canvas__expression--channel">philosophy</button>
      </div>

      <div class="canvas__expression--responses">
        <button class="canvas__expression--resonate">&nbsp;</button>
        <button class="canvas__expression--comment">&nbsp;</button>
      </div>
    </div>
</template>

<script>
export default {
    name: "ExpressionBottom",
    props: {
        channels: []
    }
}
</script>>