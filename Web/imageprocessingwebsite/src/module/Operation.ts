export default class Operation {
    constructor() {
        this.id = '' // 操作id
        this.module_name = '' // 用来存储是哪个dll的方法
        this.method_name = '' // 方法名
        this.src_img = []
        this.args = [] // 参数
    }

    toDict() {
        return {
            id: this.id,
            module_name: this.module_name,
            method_name: this.method_name,
            src_img: this.src_img,
            args: this.args
        }
    }
    id: string
    module_name: string
    method_name: string
    src_img: Array<number>
    args: any
}