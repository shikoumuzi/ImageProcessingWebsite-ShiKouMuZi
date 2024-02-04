export default class UserMsg {
    constructor() {
        this.username = ''
        this.operation_list = []
        this.history_operation_id_list = []
        this.result_image_list = []
      }
    
    setUsername(username) {
        this.username = username
    }

    clear() {
        this.username = ''
        this.operation_list = []
        this.history_operation_id_list = []
        this.result_image_list = []
    }
}