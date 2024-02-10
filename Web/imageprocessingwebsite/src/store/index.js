import { createStore } from 'vuex'
import UserMsg from '../typings/UserMsg'
 
import { ref } from 'vue';
export default createStore({
  state: {
    token: '',
    user_base_msg: new UserMsg(),
    user_login_status: false,
    
    manager_now_browser: 'home',

    brower_record: {
      from: ''
    },

    api_url: {
      login: '/login',
      register: '/register',
      checkManagerAuthority: '/checkManagerAuthority',
      checkPassword: '/checkPassword',
      resetPassword: '/resetPassword'

    },

    register_name: '',
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
    },
    getUrl(state) {
      return state.api_url
    },
    getResultImgListSize(state) {
      return state.user_base_msg.result_image_list.length
    },
    getResultImgList(state) {
      return ref(state.user_base_msg.result_image_list)
    },
    getFrom(state) {
      return state.brower_record.from
    },
    getregisterName(state) {
      return state.register_name
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
    },
    setFrom(state, from) {
      state.brower_record.from = from
    },
    // eslint-disable-next-line camelcase
    setRegisterName(state, register_name) {
      // eslint-disable-next-line camelcase
      state.register_name = register_name
    }
  },
  actions: {
  },
  modules: {
  }
})

const stroe = createStore()
