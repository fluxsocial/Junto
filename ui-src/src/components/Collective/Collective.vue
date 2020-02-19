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
        <input
          type="text"
          class="canvas__search--text"
          placeholder="search channels"
        />
      </div>

      <!-- feed -->
      <div slot="canvasFeed" class="canvas__feed">
        <div
          v-for="post in collectivePosts"
          :key="post.__ob__.dep.id"
          class="expression-list"
        >
          <!-- FOR THE STENCIL EXPRESSIONS COMPONENT -->
          <!-- <ReactExpressionsComponentStencil expression-type="post.expression.entry.expression_type" expression-data="post"></ReactExpressionsComponentStencil> -->
          
          <div v-if="post.expression.entry.expression_type == 'ShortForm'">
            <expression-top
              :users_first_name="post.author_profile.entry.first_name"
              :users_last_name="post.author_profile.entry.last_name"
              :username="post.author_username.entry.username"
              :user_address="post.author_username.address"
            />
            <short-form
              :short-form-data="{
                text: post.expression.entry.expression.ShortForm.body
              }"
            />
            <expression-bottom :channels="post.channels" />
          </div>
          <div v-if="post.expression.entry.expression_type == 'LongForm'">
            <expression-top
              :users_first_name="post.author_profile.entry.first_name"
              :users_last_name="post.author_profile.entry.last_name"
              :username="post.author_username.entry.username"
              :user_address="post.author_username.address"
            />
            <long-form
              :long-form-data="{
                title: post.expression.entry.expression.LongForm.title,
                body: post.expression.entry.expression.LongForm.body
              }"
            />
            <expression-bottom :channels="post.channels" />
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
import LongForm from "./../ExpressionViewer/LongForm/StoryPreview";
import ShortForm from "./../ExpressionViewer/Shortform/Shortform";
import ExpressionTop from "./../ExpressionViewer/ExpressionTop/ExpressionTop";
import ExpressionBottom from "./../ExpressionViewer/ExpressionBottom/ExpressionBottom";

export default {
  name: "Collective",
  components: {
    juntoNav: Nav,
    juntoLotus: Lotus,
    juntoCanvas: Canvas,
    juntoSidebarCollective: SidebarCollective,
    shortForm: ShortForm,
    longForm: LongForm,
    expressionTop: ExpressionTop,
    expressionBottom: ExpressionBottom
  },
  data() {
    return {
      collectivePosts: []
    };
  },
  mounted() {
    this.makeRandomCollectiveQuery();
  },
  methods: {
    sha256(message) {
      // encode as UTF-8
      const msgBuffer = new TextEncoder("utf-8").encode(message);
      // hash the message
      const hashBuffer = crypto.subtle.digest("SHA-256", msgBuffer);
      // convert ArrayBuffer to Array
      const hashArray = Array.from(new Uint8Array(hashBuffer));
      // convert bytes to hex string
      const hashHex = hashArray
        .map(b => ("00" + b.toString(16)).slice(-2))
        .join("");
      return hashHex;
    },
    makeRandomCollectiveQuery() {
      expressionViewerHttpMethods
        .getExpression(
          this,
          "random",
          [],
          "FilterNew",
          "ExpressionPost",
          "And",
          0,
          this.sha256(Date.now()),
          false
        )
        .then(result => {
          for (let i = 0; i < result.Ok.length; i++) {
            this.collectivePosts.push(result.Ok[i]);
            console.log(
              "Inside makeRandomCollectiveQuery",
              this.collectivePosts
            );
          }
        });
    }
  }
};
</script>
