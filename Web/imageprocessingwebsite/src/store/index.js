import { createStore } from 'vuex'
import UserMsg from '../typings/UserMsg'
 
export default createStore({
  state: {
    token: '',
    user_base_msg: new UserMsg(),
    user_login_status: false,

    manager_now_browser: 'home'
  },
  getters: {
    getDate(state) {
      return state.user_base_msg.getDate()
    },
    getUserLoginStatus(state) {
      return state.user_login_status
    },
    getUserBaseMsg(state) {
      return state.user_base_msg
    },
    getToken(state) {
      return state.token
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
    },
    clearUserMsg(state) {
      state.user_base_msg = new UserMsg()
      console.log(state)
    },
    // eslint-disable-next-line camelcase
    setUserName(state, user_name) {
      state.user_base_msg.setUserName(user_name)
    },
    setAuthority(state, authority) {
      state.user_base_msg.setAuthority(authority)
    }
  },
  actions: {
  },
  modules: {
  }
})

const stroe = createStore()
