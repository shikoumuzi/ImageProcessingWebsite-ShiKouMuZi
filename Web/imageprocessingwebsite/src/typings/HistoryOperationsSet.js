import HistoryOperation from './HistoryOperation';
import { ref } from 'vue';

const TheInitSizeOfHistoryOpertionsSet = 15

export default class HistoryOpertionsSet {
    constructor() {
        this.history_operations = []    
        this.now_vaild_size = 0
        this.init()
    }

    isNull() {
        return this.now_vaild_size === 0
    }

    init() {
        for (let i = 0; i < TheInitSizeOfHistoryOpertionsSet; ++i) {
            this.history_operations.push(new HistoryOperation())
        }
    }

    // eslint-disable-next-line camelcase
    push(history_operation) {
        if (this.now_vaild_size < TheInitSizeOfHistoryOpertionsSet) {
            // eslint-disable-next-line camelcase
            this.history_operations[this.now_vaild_size] = history_operation
        } else {
            this.history_operations.push(history_operation)
        }
        this.now_vaild_size += 1
    }

    earse(index) {
        if (index > this.now_vaild_size) {
            return
        }

        if (this.history_operations.length < TheInitSizeOfHistoryOpertionsSet) {
            for (let i = 0; i < TheInitSizeOfHistoryOpertionsSet - this.history_operations.length; ++i) {
                this.history_operations.push(new HistoryOperation())
            }
        }
        this.now_vaild_size -= 1
    }

    clear() {
        this.history_operations = []
        this.init()
    }

    toList() {
        // eslint-disable-next-line no-var, camelcase
        return ref(this.history_operations)
    }

    size() {
        return this.now_vaild_size
    }
    // history_operations_mapping: Map<number, Operation>
    // index: number
    // id: string
}