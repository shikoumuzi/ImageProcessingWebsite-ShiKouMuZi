export default class Operation {
    constructor() {
        this.operation_id = '' // 操作id
        this.module_name = '' // 用来存储是哪个dll的方法
        this.method_name = '' // 方法名
        this.args = [] // 参数
        this.output_image = [] // 输出的图像列表（占位），每个所输出的图像都会生成一个相应的占位
        this.time_stamp = new Date().getTime()
        this.mat_index = -1
    }

    toDict() {
        return {
            id: this.id,
            module_name: this.module_name,
            method_name: this.method_name,
            src_img: this.src_img,
            time_stamp: this.time_stamp,
            args: this.args
        }
    }

    // eslint-disable-next-line camelcase
    setTimeStamp(time_stamp) {
        // eslint-disable-next-line camelcase
        this.time_stamp = time_stamp
    }

    getDate() {
        return new Date(this.time_stamp)
    }

    // id: string
    // module_name: string
    // method_name: string
    // src_img: Array<number>
    // args: any
    // time_stamp: number
}