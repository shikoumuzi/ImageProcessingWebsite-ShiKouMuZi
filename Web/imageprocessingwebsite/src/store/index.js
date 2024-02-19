import { createStore } from 'vuex'
import UserMsg from '../typings/UserMsg'
 
import { h, ref } from 'vue';
import HistoryOperation from '@/typings/HistoryOperation';
import Operation from '@/typings/Operation';
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
      suggestion: '/submit_suggestion',

      operation: {
        getHistoryOperationsList: '/history_operations',
        eraseHistoryOperationList: '/earse_history_operation', 
        getOperationByHistoryOperationID: '/get_operation_details_by_history_operation_id'
      },

      manager: null

    },

    operating_msg: {
      is_operation_loaded_in_workspace: false 
    },

    abouts: {},

    register_name: '',
  },
  getters: {
    /**
     * 获取到用户的创建时间
     * @param {object} state store.state 
     * @returns 
     */
    getDate(state) {
      return state.user_base_msg.getDate()
    },

    /**
     * 获取到当前的登陆状态为是否已经登录了
     * @param {object} state store.state 
     * @returns 返回登录状态值
     */
    getUserLoginStatus(state) {
      // 获取用户
      return state.user_login_status
    },

    /**
     * 获取到用户的基本信息的引用，需要.value取值
     * @param {object} state store.state 
     * @returns 返回user_base_msg 的 ref wrapper
     */
    getUserBaseMsg(state) {
      return ref(state.user_base_msg)
    },

    /**
     * 获取到当前的toekn值
     * @param {object} state store.state 
     * @returns 返回token
     */
    getToken(state) {
      return state.token
    },

    /**
     * 获取到api Url字典
     * @param {object} state store.state 
     * @returns 返回 state.api_url
     */
    getUrl(state) {
      return state.api_url
    },

    /**
     * 获取到结果图片列表的长度
     * @param {object} state store.state 
     * @returns 返回“结果图片”的数量
     */
    getResultImgListSize(state) {
      return state.user_base_msg.result_image_list.length
    },

    /**
     * 获取到结果图片列表
     * @param {object} state store.state 
     * @returns 返回结果图片列表
     */
    getResultImgList(state) {
      // 获取结果图片列表
      return ref(state.user_base_msg.result_image_list)
    },

    /**
     * 获取到路由跳转来源
     * @param {object} state store.state 
     * @returns 返回路由跳转来源
     */
    getFrom(state) {
      // 获取来源路由
      return state.brower_record.from
    },

    /**
     * 获取到当前注册的用户名
     * @param {object} state store.state 
     * @returns 
     */
    getRegisterName(state) {
      return state.register_name
    },

    /**
     * 获取介绍的引用，需要.value
     * @param {object} state store.state 
     * @returns 返回介绍内容的 ref wrapper
     */
    getAbouts(state) {
      return ref(state.abouts)
    },

    /**
     * 获取操作历史列表
     * @param {object} state store.state 
     * @returns 返回历史操作列表
     */
    getHistoryOperationList(state) {
      return state.user_base_msg.history_operations.toList()
    },

    /**
     * 获取ImageOperationView是否存在正在操作的work space
     * @param {object} state store.state
     * @returns 返回ImageOperationView的工作状态 
     */
    getTheWorkStatusOfOperationView(state) {
      return state.operating_msg.is_operation_loaded_in_workspace
    }

  },
  mutations: {
    setToken(state, token) {
      state.token = token
    },
    /**
     * 设置用户创建时的时间戳
     * @param {object} state store.state 
     * @param {integer} time_stamp 时间戳
     */
    // eslint-disable-next-line camelcase
    setTimeStamp(state, time_stamp) {
      state.user_base_msg.setTimeStamp(time_stamp)
    },

    /**
     * 设置用户登录状态
     * @param {object} state store.state 
     * @param {boolean} status 用户登录状态
     */
    setUserLoginStatus(state, status) {
      state.user_login_status = status
    },

    /**
     * 设置用户名
     * @param {object} state store.state 
     * @param {string} user_name 用户名
     */
    // eslint-disable-next-line camelcase
    setUserName(state, user_name) {
      state.user_base_msg.setUserName(user_name)
    },

    /**
     * 设置权限值
     * @param {object} state store.state 
     * @param {integer} authority 权限值
     */
    setAuthority(state, authority) {
      state.user_base_msg.setAuthority(authority)
    },

    /**
     * 清除用户信息
     * @param {object} state store.state
     */
    clearUserMsg(state) {
      state.user_base_msg = new UserMsg()
      console.log(state)
    },

    /**
     * 设置当前路由跳转的来源路径值
     * @param {object} state store.state
     * @param {string} from 来源路径 
     */
    setFrom(state, from) {
      state.brower_record.from = from
    },

    /**
     * 设置当前注册成功后，用户的用户名
     * @param {object} state store.state 
     * @param {string} register_name 注册用户名
     */
    // eslint-disable-next-line camelcase
    setRegisterName(state, register_name) {
      // eslint-disable-next-line camelcase
      state.register_name = register_name
    },

    /**
     * 清空当前所有内容的介绍信息
     * @param {object} state store.state 
     */
    clearAbout(state) {
      state.abouts = {}
    },

    /**
     * 设置单个标题内容的介绍内容
     * @param {object} state store.state
     * @param {object} about_msg about内容{title， about}
     */
    // eslint-disable-next-line camelcase
    addAbout(state, about_msg) {
      // console.log('addAbout------------------------------')
      
      state.abouts[about_msg.title] = about_msg.about
      // console.log(state.abouts[about_msg.title])
    },

    /**
     * 设置历史操作列表信息
     * @param {object} state store.state
     * @param {array<object>} history_operations array<object>, item: {note, time_stamp, history_operation_id} 
     */
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

    /**
     * 删除索引对应的历史操作信息
     * @param {object} state store.state 
     * @param {integer} index history_operation在history_operations中的索引号
     */
    eraseHistoryOperation(state, index) {
      state.user_base_msg.history_operations.erase(index)
    },

    /**
     * 将历史操作信息添加到history_operations的尾部当中
     * @param {object} state store.state 
     * @param {object} history_operation 历史操作信息
     */
    // eslint-disable-next-line camelcase
    pushOnceToHistoryOperations(state, history_operation) {
        state.user_base_msg.history_operations.push(history_operation)
    },

    /**
     * 设置索引对应的历史操作信息当中的操作详情信息
     * @param {object} state store.state 
     * @param {integer} index history_operation在history_operations中的索引号
     * @param {array<Operation>} operation_details 该历史操作信息当中的操作详情信息 
     */
    // eslint-disable-next-line camelcase
    setOperationDetailsToOnceOfHistoryOperationByItsId(state, index, operation_details) {
    // eslint-disable-next-line camelcase
      state.user_base_msg.history_operations.history_operations[index].operations = operation_details
    },

    /**
     * 
     * @param {object} state store.state
     * @param {object} api_url 所有 管理员特有的api_url 
     */
    // eslint-disable-next-line camelcase
    setManagerApiUrl(state, api_url) {
    // eslint-disable-next-line camelcase
      state.api_url.manager = api_url
    }

  },
  actions: {
  },
  modules: {
  }
})

const stroe = createStore()
