 import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import axios from './plugin/AxiosAPI'

import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import * as ElementPlusIconVue from '@element-plus/icons-vue'

const app = createApp(App)
app.use(store).use(router)
app.use(ElementPlus)
for (const [key, component] of Object.entries(ElementPlusIconVue)) {
    app.component(key, component)
}
app.mount('#app')

// axios.defaults.baseURL = 'http://127.0.0.1:4523/m1/4019827-0-default/api'
// axios.defaults.baseURL = 'http://localhost:8080/api'
// axios.defaults.timeout = 2000
// app.prototype.$http = axios
app.config.globalProperties.$axios = axios; // 配置axios的全局引用
