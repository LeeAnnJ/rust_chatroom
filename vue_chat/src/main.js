import Vue from 'vue'
import App from './App.vue'
import './registerServiceWorker'
import router from './router'
import store from './store'
import "./assets/ifont/iconfont.css";
import axios from 'axios';
import VueAxios from 'vue-axios';
import api from './api/index'

Vue.use(VueAxios,axios);
Vue.config.productionTip = false
Vue.prototype.$api = api

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
