<template>
  <div class="lotus__container">
    <div class="lotus__post">
      <junto-lotus-header>
        <p slot="expressionType" class="lotusHeader__expressionType">
          {{ expressionType }}
        </p>
        <div
          slot="lotusChannelsPills"
          class="lotusHeader__create--channels_pills"
        >
          <ul class="channel_pills">
            <li
              v-for="channel in channels"
              :key="channel"
              class="channel_pills_item"
            >
              {{ channel }}
              <span class="remove-channel" @click="removeChannel(channel)">
                <svg class="remove-icon">
                  <use xlink:href="../../../../src/assets/img/sprite.svg#icon-cross"></use>
                </svg>
              </span>
            </li>
          </ul>
        </div>
        <input
          slot="lotusChannels"
          v-model="channel"
          type="text"
          placeholder="Add Channels"
          class="lotusHeader__create--channels"
          @keyup.enter="addChannel"
        />
        <Button
          v-if="!this.creating"
          slot="lotusCreate"
          active-class="lotusHeader__create--button"
          :method="createExpression"
          text="Create"
        >
        </Button>
        <div
          v-if="this.creating"
          slot="loadingPostExpression"
          class="loading-spinner"
        >
          <svg
            class="spinner"
            style="background: rgba(0, 0, 0, 0) none repeat scroll 0% 0%; display: block; shape-rendering: auto; width: 5rem; height:5rem;"
          >
            <use
              xlink:href="../../../../src/assets/img/sprite.svg#loading"
            ></use>
          </svg>
        </div>
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
        <use xlink:href="../../../src/assets/img/sprite.svg#icon-lotusicon" />
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
import LotusHttp from "../LotusHttp.js";
import LotusShortformVue from "./../LotusExpressions/LotusShortform/LotusShortform.vue";
import Button from "../../Button/Button";

export default {
  components: {
    juntoLotusHeader: LotusHeader,
    juntoLotusFooter: LotusFooter,
    juntoLotusStory: LotusStory,
    juntoLotusShortform: LotusShortform,
    juntoLotusPhoto: LotusPhoto,
    Button: Button
  },
  data() {
    return {
      storyOpen: true,
      shortformOpen: false,
      photoOpen: false,
      eventsOpen: false,
      expressionType: "STORY",
      shortFormChild: undefined,
      channel: null,
      channels: [],
      postingError: [],
      creating: false
    };
  },

  methods: {
    addChannel() {
      if (this.channels.length >= 4) {
        this.$notify({
          type: "error",
          group: "main",
          title: "Too many channels tagged",
          text:
            "You have reached the limit on channels tagged for this expression",
          duration: 5000
        });
      } else {
        this.channels.push(this.channel);
        this.channel = "";
      }
    },
    removeChannel(channelName) {
      const index = this.channels.indexOf(channelName);
      if (index > -1) {
        this.channels.splice(index, 1);
      }
    },
    createExpression() {
      if (this.storyOpen == true) {
        let child = this.$children[1]; //This might not be the right way to do this
        if (!child.title && !child.innerHtml) {
          if (!child.title)
            this.postingError.push("Title required for longform expression");
          if (!child.innerHtml)
            this.postingError.push("Body required for longform expression");
        } else {
          this.creating = true;
          let expression_data = {
            expression: {
              LongForm: {
                title: child.title,
                body: child.innerHtml
              }
            },
            expression_type: "LongForm"
          };
          console.log("Creating long form expression with", expression_data);
          LotusHttp.createExpression(
            this,
            expression_data,
            this.$store.getters.getDnaAddress,
            this.channels
          ).then(result => {
            console.log("Added expression to holochain with result", result);
            this.$router.push("/");
          });
        }
      } else if (this.shortformOpen == true) {
        let child = this.$children[2]; //This might not be the right way to do this
        if (!child.text) {
          this.postingError.push("Body required for shortform expression");
        } else {
          this.creating = true;
          let expression_data = {
            expression: {
              ShortForm: {
                background: child.whichBackground().toString(),
                body: child.text
              }
            },
            expression_type: "ShortForm"
          }; //Length of the text should be validated
          console.log("Creating short form expression with", expression_data);
          //Should not pass this but instead just the store object itself
          LotusHttp.createExpression(
            this,
            expression_data,
            this.$store.getters.getDnaAddress,
            this.channels
          ).then(result => {
            console.log("Added expression to holochain with result", result);
            this.$router.push("/");
          });
        }
      } else if (this.photoOpen == true) {
      } else if (this.eventsOpen == true) {
      }
      for (let i = 0; i < this.postingError.length; i++) {
        this.$notify({
          type: "error",
          group: "main",
          title: "Error posting expression",
          text: this.postingError[i],
          duration: 5000
        });
      }
      console.log("Awaiting holochain response");
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
