import { createStore } from 'vuex'
import UserMsg from '../typings/UserMsg'
 
import { h, ref } from 'vue';
import HistoryOperation from '@/typings/HistoryOperation';
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
      user: {
        login: '/login',
        register: '/register',
        checkManagerAuthority: '/checkManagerAuthority',
        checkPassword: '/checkPassword',
        resetPassword: '/resetPassword',
      },
      about: '/about',
      suggestion: '/suggestion',

      operation: {
        getHistoryOperationsList: '/history_operations',
        eraseHistoryOperationList: '/earse_history_operation', 
        getOperationByHistoryOperationID: '/get_operation_details_by_history_operation_id'
      },

    },

    operating_msg: {
      is_operation_loaded_in_workspace: false 
    },

    abouts: {},

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
      return ref(state.user_base_msg)
    },
    getToken(state) {
      return state.token
    },
    getUrl(state) {
      // 获取api的url
      return state.api_url
    },
    getResultImgListSize(state) {
      // 获取已经生成的结果图片数量
      return state.user_base_msg.result_image_list.length
    },
    getResultImgList(state) {
      // 获取结果图片列表
      return ref(state.user_base_msg.result_image_list)
    },
    getFrom(state) {
      // 获取来源路由
      return state.brower_record.from
    },
    getRegisterName(state) {
      return state.register_name
    },
    getAbouts(state) {
      return ref(state.abouts)
    },
    getHistoryOperationList() {
      return ref()
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
    // eslint-disable-next-line camelcase
    setUserName(state, user_name) {
      state.user_base_msg.setUserName(user_name)
    },
    setAuthority(state, authority) {
      state.user_base_msg.setAuthority(authority)
    },
    clearUserMsg(state) {
      state.user_base_msg = new UserMsg()
      console.log(state)
    },

    setFrom(state, from) {
      state.brower_record.from = from
    },
    // eslint-disable-next-line camelcase
    setRegisterName(state, register_name) {
      // eslint-disable-next-line camelcase
      state.register_name = register_name
    },
    clearAbout(state) {
      state.abouts = {}
    },
    // eslint-disable-next-line camelcase
    addAbout(state, about_msg) {
      // console.log('addAbout------------------------------')
      
      state.abouts[about_msg.title] = about_msg.about
      // console.log(state.abouts[about_msg.title])
    },
    // eslint-disable-next-line camelcase
    setHistoryOperations(state, history_operations) {
      state.user_base_msg.history_operations.clear()
      for (let i = 0; i < history_operations.length; ++i) {
        // eslint-disable-next-line camelcase, prefer-const
        let history_operation = new HistoryOperation()
        history_operation.note = history_operations[i].note
        history_operation.time_stamp = history_operations[i].time_stamp
        history_operation.history_operation_id = history_operations[i].history_operation_id
        state.user_base_msg.history_operations.push(history_operation)
      }
    },
    eraseHistoryOperation(state, index) {
      state.user_base_msg.history_operations.erase(index)
    },
    // eslint-disable-next-line camelcase
    pushOnceToHistoryOperations(state, history_operation) {
        state.user_base_msg.history_operations.push(history_operation)
    }
  },
  actions: {
  },
  modules: {
  }
})

const stroe = createStore()
