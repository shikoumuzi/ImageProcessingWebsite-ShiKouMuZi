 import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import axios from './plugin/AxiosAPI'

const app = createApp(App)
app.use(store).use(router).mount('#app')
app.config.globalProperties.$axios = axios; // 配置axios的全局引用