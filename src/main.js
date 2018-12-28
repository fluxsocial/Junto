import Vue from 'vue'
import VueRouter from 'vue-router'
import { routes } from './routes'

// Froala
  // Require Froala Editor js file.
  require('froala-editor/js/froala_editor.pkgd.min')

  // Require Froala Editor css files.
  require('froala-editor/css/froala_editor.pkgd.min.css')
  require('font-awesome/css/font-awesome.css')
  require('froala-editor/css/froala_style.min.css')
  require('froala-editor/css/junto.css')
  

  // import Froala
  import VueFroala from 'vue-froala-wysiwyg'
  Vue.use(VueFroala);


import App from './App.vue'


Vue.use(VueRouter);

const router = new VueRouter({
  mode: 'history',
  routes
})

new Vue({
  el: '#app',
  router,
  render: h => h(App)
})
