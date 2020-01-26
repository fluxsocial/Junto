<template>
  <div class="lotus__container">
    <div class="lotus__post">
      <junto-lotus-header>
        <p slot="expressionType" class="lotusHeader__expressionType">
          {{ expressionType }}
        </p>
        <button
          slot="lotusCreate"
          class="lotusHeader__create--button"
          @click="createExpression"
        >
          create
        </button>
      </junto-lotus-header>

      <junto-lotus-story v-if="storyOpen"></junto-lotus-story>
      <junto-lotus-shortform v-if="shortformOpen"></junto-lotus-shortform>
      <junto-lotus-photo v-if="photoOpen"></junto-lotus-photo>
      <junto-lotus-photo v-if="eventsOpen"></junto-lotus-photo>
    </div>
    <junto-lotus-footer>
      <svg
        slot="lotus"
        class="lotusFooter__lotus"
        :class="{ storyActive: storyOpen, shortformActive: shortformOpen }"
        @click="closeLotus"
      >
        <use
          xlink:href="../../../src/assets/img/sprite.svg#icon-lotusicon"
        ></use>
      </svg>
      <button
        slot="lotusStory"
        class="lotusFooter__expression"
        :class="{ 'lotusFooter__expression--story': storyOpen }"
        @click="openStory"
      ></button>
      <button
        slot="lotusShortform"
        class="lotusFooter__expression"
        :class="{ 'lotusFooter__expression--shortform': shortformOpen }"
        @click="openShortform"
      ></button>
      <button
        slot="lotusPhoto"
        class="lotusFooter__expression"
        @click="openPhoto"
      >
        &nbsp;
      </button>
      <button
        slot="lotusEvents"
        class="lotusFooter__expression"
        @click="openEvents"
      >
        &nbsp;
      </button>
    </junto-lotus-footer>
  </div>
</template>

<script>
import VueFroala from "vue-froala-wysiwyg";
import LotusHeader from "./../LotusHeader/LotusHeader.vue";
import LotusFooter from "./../LotusFooter/LotusFooter.vue";
import LotusStory from "./../LotusExpressions/LotusStory/LotusStory.vue";
import LotusShortform from "./../LotusExpressions/LotusShortform/LotusShortform.vue";
import LotusPhoto from "./../LotusExpressions/LotusPhoto/LotusPhoto.vue";
import LotusEvents from "./../LotusExpressions/LotusEvents/LotusEvents.vue";

export default {
  components: {
    juntoLotusHeader: LotusHeader,
    juntoLotusFooter: LotusFooter,
    juntoLotusStory: LotusStory,
    juntoLotusShortform: LotusShortform,
    juntoLotusPhoto: LotusPhoto
  },
  data() {
    return {
      storyOpen: true,
      shortformOpen: false,
      photoOpen: false,
      eventsOpen: false,
      expressionType: "STORY"
    };
  },

  methods: {
    createExpression() {
      if (this.storyOpen == true) {

      } else if (this.shortformOpen == true) {

      } else if (this.photoOpen == true) {

      } else if (this.eventsOpen == true) {

      };
      console.log("I created an expression !");
    },

    closeLotus() {
      this.$router.go(-1);
    },

    openStory() {
      this.storyOpen = true;
      this.shortformOpen = false;
      this.photoOpen = false;
      this.eventsOpen = false;
      this.expressionType = "STORY";
    },

    openShortform() {
      this.storyOpen = false;
      this.shortformOpen = true;
      this.photoOpen = false;
      this.eventsOpen = false;
      this.expressionType = "SHORTFORM";
    },

    openBullet() {
      this.storyOpen = false;
      this.shortformOpen = false;
      this.bulletOpen = true;
      this.photoOpen = false;
      this.eventsOpen = false;
      this.expressionType = "BULLET";
    },

    openPhoto() {
      this.storyOpen = false;
      this.shortformOpen = false;
      this.photoOpen = true;
      this.eventsOpen = false;
      this.expressionType = "PHOTO";
    },

    openEvents() {
      this.storyOpen = false;
      this.shortformOpen = false;
      this.photoOpen = false;
      this.eventsOpen = true;
      this.expressionType = "EVENTS";
    },

    openMusic() {
      this.storyOpen = false;
      this.shortformOpen = false;
      this.photoOpen = false;
      this.eventsOpen = false;
      this.musicOpen = true;
      this.expressionType = "MUSIC";
    }
  }
};
</script>

<style lang="scss">
@import "./../../../../sass/base";

.storyActive {
  fill: $junto-primary;
}

.shortformActive {
  fill: $junto-secondary;
}
</style>
