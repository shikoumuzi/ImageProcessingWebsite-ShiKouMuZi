<!-- eslint-disable vue/multi-word-component-names -->
<template>
  <div>
    <el-form ref="form" :model="form" :rules="rules" label-width="80px">
      <el-form-item label="图像A" prop="img_a">
        <el-input placeholder="请填写图像编号" :maxLength="20" v-model="form.img_a" />
      </el-form-item>
      <el-form-item label="w" prop="w">
        <el-input placeholder="请填写 宽 值" :maxLength="20" v-model="form.e" />
      </el-form-item>
      <el-form-item label="h" prop="h">
        <el-input placeholder="请填写 高 值" :maxLength="20" v-model="form.h" />
      </el-form-item>

    </el-form>
    <el-button type="primary" size="large" @click="submit">提交到服务器</el-button>
  </div>
</template>

<script>
import axios from '../../../plugin/AxiosAPI'
import mitt from '../../../plugin/MittAPI'
export default {
    data() {
        return {
            form: {
                img_a: -1,
                w: -1,
                h: -1
            },
            rules: [
                {}
            ],
            mode: '',
            result_index: ''
        }
    },
    mounted() {
      mitt.on('mode', (res) => {
        this.mode = res
      })
    },
    methods: {
      submit() {
          axios.get(this.$store.getters.getUrl.operation.operation.mat.resize, {
          params: {
            token: this.$store.getters.getToken,
            img_a: this.form.img_a,
            h: this.form.h,
            w: this.form.w
          }
          }).then(response => {
            if (response.data != null) {
              if (response.data.status != null) {
                console.log('ok')
                this.result_index = response.data.mat_index
                mitt.emit('result_index', this.result_index)
              }
            }
          })
      }
        
    }
}
</script>

<style>

</style>