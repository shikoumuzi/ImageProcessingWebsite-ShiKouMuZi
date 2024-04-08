<template>
  <div>
    <el-form ref="form" :model="form" :rules="rules" label-width="80px">
      <el-form-item label="图像A" prop="img_a">
        <el-input placeholder="请填写图像编号" :maxLength="20" v-model="form.img_a" />
      </el-form-item>
      <el-form-item label="图像B" prop="img_b">
        <el-input placeholder="请填写图像编号" :maxLength="20" v-model="form.img_b" />
      </el-form-item>
    </el-form>
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
                img_b: -1
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
        if (this.mode === 'h') {
          axios.get(this.$store.getters.getUrl.operation.operation.mat.hstack, {
          params: {
            token: this.$store.getters.getToken,
            mat_index_vec: [this.form.img_a, this.form.img_b]
          }
          }).then(response => {
            if (response.data != null) {
              if (response.data.status != null) {
                console.log('ok')
                this.result_index = response.mat_index
                mitt.emit('result_index', this.result_index)
              }
            }
          })
      } else {
        axios.get(this.$store.getters.getUrl.operation.operation.mat.vstack, {
          params: {
            token: this.$store.getters.getToken,
            mat_index: this.form.img_a
          }
          }).then(response => {
            if (response.data != null) {
              if (response.data.status != null) {
                console.log('ok')
                this.result_index = response.mat_index
                mitt.emit('result_index', this.result_index)
              }
            }
          })
      }
    }
        
    }
}
</script>

<style>

</style>