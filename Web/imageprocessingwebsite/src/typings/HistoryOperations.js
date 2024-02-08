import Operation from './Operation'

export default class HistoryOpertions {
    constructor() {
        this.index = 1
        this.id = ''
    }

    push(operation) {
        this.history_operations_mapping[this.index] = operation
        this.index = this.index + 1
    }

    clear() {
        this.history_operations_mapping.clear()
        this.index = 1
        this.id = ''
    }

    toList() {
        // eslint-disable-next-line no-var, camelcase
        var opertion_list = []        
        for (let i = 0; i < this.history_operations_mapping.size; ++i) {
            opertion_list.push(this.history_operations_mapping[i])
        }
        // eslint-disable-next-line camelcase
        return opertion_list
    }
    // history_operations_mapping: Map<number, Operation>
    // index: number
    // id: string
}