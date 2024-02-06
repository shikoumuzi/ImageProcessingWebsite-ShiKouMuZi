import { createStore } from 'vuex'
import UserMsg from '../typings/UserMsg'
 
export default createStore({
  state: {
    token: '',
    user_base_msg: UserMsg(),
    user_login_status: false,
    manager_now_browser: 'home'
  },
  getters: {
    getDate(state) {
      return state.user_base_msg.getDate()
    },
    getUserLoginStatus(state) {
      return state.user_login_status
    }
  },
  mutations: {
    setToken(state, token) {
      state.token = token
    },
    // eslint-disable-next-line camelcase
    setTimeStamp(state, time_stamp) {
      state.user_base_msg.setTimeStamp(time_stamp)
    },
    setUserLoginStatus(state, status) {
      state.user_login_status = status
    }
  },
  actions: {
  },
  modules: {
  }
})

const stroe = createStore()
