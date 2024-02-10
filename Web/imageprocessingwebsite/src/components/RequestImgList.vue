<template>
  <div>
    <ul>
        <li v-for="index in this.result_img_count" :key="index">
            <div style="display: flex; flex-direction: column; justify-content: center;">
                <el-image :src="getImgSrc(i)" :fit="contain" />
                <div style="display: flex; justify-content: flex-end;">
                    <span style="margin-right: 2pt;">{{ i }}</span>

                    <div>
                        <el-icon><Download /></el-icon>
                    </div>

                </div>
            </div>
        </li>
    </ul>
  </div>
</template>

<script>
import { watch } from 'vue'
export default {
    data () {
        return {
            result_img_count: 0 
            
        }
    },
    methods: {
        getImgSrc(index) {
            return window.URL.createObjectURL(((this.$store.getters.getResultImgList).value)[index].img)
        },
    },
    watch: {
        '$store.state.user_base_msg.result_image_list.length'(newValue, oldValue) {
            console.log('work')
            this.result_img_count = newValue
        },
    }
}
</script>

<style>

</style>