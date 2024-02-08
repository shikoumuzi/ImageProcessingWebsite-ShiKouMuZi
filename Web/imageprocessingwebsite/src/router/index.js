import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import AboutView from '../views/AboutView.vue'
import LoginView from '../views/LoginView.vue'
import RegisterView from '../views/RegisterView.vue';
import ImageOperationView from '../views/ImageOperationView.vue'
import UserView from '../views/UserView.vue'
import ManagerView from '@/views/ManagerView.vue';
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
    component: ImageOperationView
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

  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

router.beforeEach((to, from, next) => {
  if (to.path === '/image_operation') {
    if (store.getters.getUserBaseMsg.authority === 0) {
      // eslint-disable-next-line quotes
      next("/login")
    }
  }
  if (to.path === '/manager') {
    if (store.getters.getUserBaseMsg.authority !== 2) {
      next('/home')
    }
    axios.post('/api/checkManagerAuthority', 
    {
      params: {
        token: store.getters.getToken
      }
    }).then((res) => {
      if (res.data !== null) {
        if (res.data.state !== 0) {
          next('/home')
        }
      }
    })
  }
  next()
})

export default router
