import HistoryOpertions from './HistoryOperations'
import Operation from './Operation'
import Img from './Img'

export default class UserMsg {
    constructor() {
        this.username = ''
        this.operation_list = []
        this.history_operations = new HistoryOpertions()
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
    // username: string
    // operation_list: Array<Operation>
    // history_operations: HistoryOpertions
    // result_image_list: Array<Img>
}