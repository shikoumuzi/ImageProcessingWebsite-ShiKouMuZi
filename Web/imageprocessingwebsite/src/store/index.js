import { createStore } from 'vuex'
import { UserMsg } from '@/module/UserMsg'
import { Image } from '@/module/Img'
 
export default createStore({
  state: {
    token: '',
    user_base_msg: UserMsg(),
    is_user_exited: false,
  },
  getters: {
    getDate(state) {
      return state.user_base_msg.getDate()
    }
  },
  mutations: {
    setToken(state, token) {
      state.token = token
    },
    // eslint-disable-next-line camelcase
    setTimeStamp(state, time_stamp) {
      state.user_base_msg.setTimeStamp(time_stamp)
    }
  },
  actions: {
  },
  modules: {
  }
})

const stroe = createStore()
