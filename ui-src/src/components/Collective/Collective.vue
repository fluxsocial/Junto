<template>
  <section class="collective">
    <!-- nav -->
    <junto-nav class="navigation--collective">
      <img
        slot="navigationLogo"
        class="navigation__top--logo"
        src="./../../assets/img/junto-web__logo--blue.png"
      />
      <!-- <div slot="navigationBottom" class="navigation__bottom navigation__bottom--collective">
                <p class="navigation__bottom--space">JUNTO</p>
      </div>-->
      <div slot="navigationBorder" class="navigation__border"></div>
    </junto-nav>

    <junto-sidebar-collective></junto-sidebar-collective>

    <!-- collective canvas -->
    <junto-canvas>
      <!-- search -->
      <div slot="canvasSearch" class="canvas__search">
        <input type="text" class="canvas__search--text" placeholder="search channels" />
      </div>

      <!-- feed -->
      <div slot="canvasFeed" class="canvas__feed">
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

          <img src="./../../assets/img/junto-web__sacred.png" alt class="canvas__photo--photo" />
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

        <!-- story expression -->
        <div class="canvas__expression canvas__story">
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
          <p class="canvas__story--title">The Medium is the Message</p>
          <p class="canvas__story--body">
            Hi. Before we start, I should let you know we’re going to be
            intentionally vague about our specific product features (for now).
            The purpose of this article is to shed light on our core design
            philosophy, why the current..
          </p>

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
      </div>
    </junto-canvas>

    <!-- create -->
    <junto-lotus>
      <svg slot="lotusIcon" class="lotus__icon lotus__icon--collective">
        <use xlink:href="../../../src/assets/img/sprite.svg#icon-lotusicon" />
      </svg>
    </junto-lotus>
  </section>
</template>

<script>
import Lotus from "./../Lotus/Lotus.vue";
import Nav from "./../Nav/Nav.vue";
import Canvas from "./../Canvas/Canvas.vue";
import SidebarCollective from "./../Sidebar/SidebarCollective/SidebarCollective.vue";
import expressionViewerHttpMethods from "./../ExpressionViewer/ExpressionViewerHttp";

export default {
  name: "collective",
  components: {
    juntoNav: Nav,
    juntoLotus: Lotus,
    juntoCanvas: Canvas,
    juntoSidebarCollective: SidebarCollective
  },
  data() {
    return {
      collectivePosts: []
    }
  },
  mounted() {
    this.makeRandomCollectiveQuery();
  },
  methods: {
    sha256(message) {
      // encode as UTF-8
      const msgBuffer = new TextEncoder('utf-8').encode(message);                    
      // hash the message
      const hashBuffer = crypto.subtle.digest('SHA-256', msgBuffer);
      // convert ArrayBuffer to Array
      const hashArray = Array.from(new Uint8Array(hashBuffer));
      // convert bytes to hex string                  
      const hashHex = hashArray.map(b => ('00' + b.toString(16)).slice(-2)).join('');
      return hashHex;
    },
    makeRandomCollectiveQuery() {
      expressionViewerHttpMethods.getExpression(
        this, 
        "random",
        [],
        "FilterNew",
        "ExpressionPost",
        "And",
        0,
        this.sha256(Date.now()),
        false
      ).then(result => {
        for(let i = 0; i < result.Ok.length; i++) {
          this.collectivePosts.push(result.Ok[i]);
          console.log(this.collectivePosts);
        }
      })
    }
  }
};
</script>
