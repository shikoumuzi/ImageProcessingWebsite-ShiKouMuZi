import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import AboutView from '../views/AboutView.vue'
import LoginView from '../views/user/LoginView.vue'
import RegisterView from '../views/user/RegisterView.vue';
import ImageOperationView from '../views/ImageOperationView.vue'
import UserView from '../views/UserView.vue'
import store from '../store/index';
import axios from 'axios'
import { ElNotification } from 'element-plus';
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
    component: () => import('../views/ManagerView.vue')

  },
  {
    path: '/history_operation',
    name: 'history_operation',
    component: () => import('../views/HistoryOperationsView.vue')

  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

router.beforeEach((to, from, next) => { 
  store.commit('setFrom', from.path)
  // 检查前往操作的路由跳转
  if (to.path === '/image_operation' || to.path === '/history_operation') {
    store.commit('setFrom', '/image_operation')
    // console.log(store.getters.getFrom)
    if (store.getters.getUserBaseMsg.value.authority === 0) {
      // eslint-disable-next-line quotes
      ElNotification.error({
        title: '错误',
        message: '请先登录',
        duration: 4000,
      })
      next('/login')
    }
  }
  // 检查前往登录的界面跳转
  if (to.path === '/manager') {
    if (store.getters.getUserBaseMsg.value.authority !== 2) {
      ElNotification.error({
        title: '错误',
        message: '权限不足',
        duration: 4000,
      })
      next('/home')
    }
    
    axios.post(store.getters.getUrl.user.checkManagerAuthority, 
    {
      params: {
        token: store.getters.getToken
      }
    }).then((res) => {
      if (res.data !== null) {
        if (res.data.status !== 0) {
          ElNotification.error({
            title: '错误',
            message: '权限不足',
            duration: 4000,
          })
          
          next('/home')
        }
      }
    })
  }
// 检查前往用户界面
if (to.path === '/user') {
  if (store.getters.getUserBaseMsg.value.authority !== 0 && store.getters.getUserBaseMsg.value.token !== '') {
    next()
  } else {
    ElNotification.error({
      title: '错误',
      message: '请先登录',
      duration: 4000,
    })
    next('/home')
  }
}

  next()
})

export default router
