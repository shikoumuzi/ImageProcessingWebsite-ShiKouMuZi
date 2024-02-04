import Operation from "./Operation"

export default class HistoryOpertions {
    constructor() {
        this.index = 1
        this.id = ''
    }
    push(operation: Operation) {
        this.history_operations_mapping[this.index] = operation
        this.index = this.index + 1
    }
    toList() {
        var opertion_list = []        
        for(var i = 0; i < this.history_operations_mapping.size; ++i)
        {
            opertion_list.push(this.history_operations_mapping[i])
        }
        return opertion_list
    }
    history_operations_mapping: Map<number, Operation>
    index: number
    id: string
}