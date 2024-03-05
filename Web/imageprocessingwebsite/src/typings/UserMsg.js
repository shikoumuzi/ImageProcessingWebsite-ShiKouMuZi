import HistoryOpertionsSet from './HistoryOperationsSet'
import Operation from './Operation'
import Img from './Img'
import { ref } from 'vue';
export default class UserMsg {
    constructor() {
        this.username = ''  
        this.operation_list = []
        this.history_operations = new HistoryOpertionsSet()
        this.result_image_list = []
        this.authority = 0
        this.time_stamp = 0
      }
    
    setUserName(username) {
        this.username = username
    }

    setAuthority(authority) {
        this.authority = authority
    }

    // eslint-disable-next-line camelcase
    setTimeStamp(time_stamp) {
        // eslint-disable-next-line camelcase
        this.time_stamp = time_stamp
    }

    clear() {
        this.username = ''
        this.operation_list = []
        this.history_operations.clear()
        this.result_image_list = []
        this.authority = 0
    }

    getHistoryOperations() {
        console.log('getHistoryOperations')
        console.log(this.history_operations)
        return this.history_operations
    }
}