<template>
  <div>
    <ul>
        <li v-for="index in this.result_img_count" :key="index">
            <el-image :src="getImgSrc(i)" :fit="contain" />
            <div style="display: flex; justify-content: flex-end;">
                <span>{{ i }}</span>

                <div>
                    <i class="el-icon-" />
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
            return window.URL.createObjectURL((this.$store.getters.getResultImgList)[index].img)
        },
    },
    watch: {
        '$store.state.user_base_msg.result_image_list.length'(newValue, oldValue) {
            this.result_img_count = newValue
        }
    }
}
</script>

<style>

</style>