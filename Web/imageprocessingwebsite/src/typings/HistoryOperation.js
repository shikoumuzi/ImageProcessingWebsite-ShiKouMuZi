import Operation from './Operation'
import { ref } from 'vue';
export default class HistoryOperation {
    constructor() {
        this.operations = null
        this.time_stamp = 0
        this.note = ''
        this.history_operation_id = ''
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