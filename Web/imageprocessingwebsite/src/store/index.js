import { createStore } from 'vuex'
import { UserMsg } from '@/module/UserMsg'
import { Image } from '@/module/Img'
 
export default createStore({
  state: {
    token: '',
    user_base_msg: UserMsg(),

  },
  getters: {
  },
  mutations: {
  },
  actions: {
  },
  modules: {
  }
})
