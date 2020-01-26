<template>
  <!-- story expression -->
  <div class="canvas__expression canvas__photo">
    <!-- expression top -->
    <div class="canvas__expression--top">
      <div class="canvas__expression--profile">
        <button class="canvas__expression--profile--picture">&nbsp;</button>

        <div class="canvas__expression--profile--details">
          <p class="canvas__expression--profile--name">Eric Yang</p>
          <p class="canvas__expression--profile--handle">@sunyata</p>
        </div>
      </div>
      <p class="canvas__expression--edit">edit</p>
    </div>

    <!-- expression story -->

    <img
      src="./../../assets/img/junto-web__sacred.png"
      alt
      class="canvas__photo--photo"
    />
    <p class="canvas__photo--caption">Livin</p>

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
