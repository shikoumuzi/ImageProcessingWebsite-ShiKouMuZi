/* eslint-disable camelcase */
import axios from '../plugin/AxiosAPI'

export default class ImagePlaceholder {
    constructor() {
        this.symbol_id = '' // 标识符号 通过 time_stamp user_id 本机IP地址
        this.src_operation_id = '' // 来源操作id
    }

    initSymbolID(user_id) {
        axios.get(
            'https://api.ipify.org/?format=json'
        ).then((response) => {
            if (response.data != null) {
                const ip = response.data.ip
                const time_stamp = new Date().valueOf().toString()
                this.symbol_id = user_id + '|' + time_stamp + '|' + ip
            }
        })
    }

    setSrcOperationID(src_operation_id) {
        this.src_operation_id = src_operation_id
    }
}