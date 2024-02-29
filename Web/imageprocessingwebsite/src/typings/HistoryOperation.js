import Operation from './Operation'
import { ref } from 'vue';
export default class HistoryOperation {
    // 历史操作的存储逻辑为 首先填写预设变量（所有不在操作链的变量都需要存储在这），然后在底下任何一个操作中都可能会作为实参传入
    // 然后每个t时刻做出的操作所输出的图像会作为一个占位，方便t+n时刻的图像随时作为实参传入进行处理
    constructor() {
        this.init_args = {} // 输入参数
        this.operations = null // 操作集合 列表存储
        this.time_stamp = 0 // 创建时间
        this.note = '' // 备注
        this.history_operation_id = '' // 标识id
    }

    push(opeartion) {
        this.operations.push(opeartion)
    }

    clear() {
        this.operations.clear()
    }

    get(index) {
        return ref(this.operations[index])
    }

    isNotStoreOperations() {
        return this.operations === null
    }
}