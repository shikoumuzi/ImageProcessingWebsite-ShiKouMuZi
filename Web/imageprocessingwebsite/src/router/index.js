import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import AboutView from '../views/AboutView.vue'
import LoginView from '../views/user/LoginView.vue'
import RegisterView from '../views/user/RegisterView.vue';
import ImageOperationView from '../views/ImageOperationView.vue'
import UserView from '../views/UserView.vue'
import ManagerView from '@/views/ManagerView.vue';
import HistoryOperationVue from '@/views/manager/HistoryOperation.vue';
import store from '../store/index';
import axios from 'axios'
const routes = [
  {
    path: '/',
    name: 'home',
    component: HomeView
  },
  {
    path: '/about',
    name: 'about',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: AboutView
  },
  {
    path: '/login',
    name: 'login',
    component: LoginView
  },
  {
    path: '/register',
    name: 'register',
    component: RegisterView
  },
  {
    path: '/image_operation',
    name: 'image_operation',
    component: ImageOperationView,
  },
  {
    path: '/user',
    name: 'user',
    component: UserView
  },
  {
    path: '/manager',
    name: 'manager',
    component: ManagerView

  },
  {
    path: '/history_operation',
    name: 'history_operation',
    component: HistoryOperationVue

  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

router.beforeEach((to, from, next) => {
  store.commit('setFrom', from.path)
  // 检查前往操作的路由跳转
  if (to.path === '/image_operation') {
    store.commit('setFrom', '/image_operation')
    // console.log(store.getters.getFrom)
    if (store.getters.getUserBaseMsg.authority === 0) {
      // eslint-disable-next-line quotes
      next("/login")
    }
  }
  // 检查前往登录的界面跳转
  if (to.path === '/manager') {
    if (store.getters.getUserBaseMsg.authority !== 2) {
      next('/home')
    }
    
    axios.post(store.getters.getUrl.checkManagerAuthority, 
    {
      params: {
        token: store.getters.getToken
      }
    }).then((res) => {
      if (res.data !== null) {
        if (res.data.status !== 0) {
          next('/home')
        }
      }
    })
  }
// 检查前往用户界面
if (to.path === '/user') {
  if (store.getters.getUserBaseMsg.authority !== 0 && store.getters.getUserBaseMsg.token !== '') {
    next()
  } else {
    next('/home')
  }
}

  next()
})

export default router
