<template>
  <section class="pack">
    <junto-nav class="navigation--pack">
      <img
        slot="navigationLogo"
        class="navigation__top--logo"
        src="./../../assets/img/junto-web__logo--purple.png"
      />
      <!-- <div slot="navigationBottom" class="navigation__bottom navigation__bottom--pack">
                <p class="navigation__bottom--space">PACK</p>
            </div> -->

      <div slot="navigationBorder" class="navigation__border"></div>
    </junto-nav>

    <junto-sidebar-pack></junto-sidebar-pack>

    <junto-canvas>
      <!-- search -->
      <div slot="canvasSearch" class="canvas__search">
        <input
          type="text"
          class="canvas__search--text"
          placeholder="search channels"
        />
      </div>

      <!-- feed -->
      <div slot="canvasFeed" class="canvas__feed">
        <!-- story expression -->
        <div class="canvas__expression canvas__story">
        </div>
      </div>
    </junto-canvas>

    <junto-lotus>
      <svg slot="lotusIcon" class="lotus__icon lotus__icon--pack">
        <use
          xlink:href="../../../src/assets/img/sprite.svg#icon-lotusicon"
        ></use>
      </svg>
    </junto-lotus>
  </section>
</template>

<script>
import Nav from "./../Nav/Nav.vue";
import Lotus from "./../Lotus/Lotus.vue";
import SidebarPack from "./../Sidebar/SidebarPack/SidebarPack.vue";
import Canvas from "./../Canvas/Canvas.vue";

export default {
  components: {
    juntoNav: Nav,
    juntoLotus: Lotus,
    juntoSidebarPack: SidebarPack,
    juntoCanvas: Canvas
  }
};
</script>
