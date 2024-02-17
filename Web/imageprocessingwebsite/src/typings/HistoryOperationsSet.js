
import { ref } from 'vue';
export default class HistoryOpertionsSet {
    constructor() {
        this.history_operations = []    
    }

    // eslint-disable-next-line camelcase
    push(history_operation) {
        this.history_operations.push(history_operation)
    }

    clear() {
        this.history_operations_mapping.clear()
    }

    toList() {
        // eslint-disable-next-line no-var, camelcase
        return ref(this.history_operations)
    }
    // history_operations_mapping: Map<number, Operation>
    // index: number
    // id: string
}