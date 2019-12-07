import Vue from "vue";
import Vuex from "vuex";
import VueRouter from "vue-router";
import { routes } from "./routes";
import { store } from "./store.js";
import 'es6-promise/auto'

Vue.use(Vuex);

// import Froala
import VueFroala from "vue-froala-wysiwyg";

import App from "./App.vue";

// Froala
// Import Froala Editor js file.
import "froala-editor/js/froala_editor.pkgd.min";

// Import Froala Editor css files.
import "froala-editor/css/froala_editor.pkgd.min.css";
import "font-awesome/css/font-awesome.css";
import "froala-editor/css/froala_style.min.css";
// import "froala-editor/css/junto.css";
Vue.use(VueFroala);

Vue.use(VueRouter);

const router = new VueRouter({
  mode: "history",
  routes
});

new Vue({
  el: "#app",
  router,
  store,
  render: h => h(App)
});
