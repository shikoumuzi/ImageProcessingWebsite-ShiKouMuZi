<template>
    <div>
      <el-form ref="form" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="图像A" prop="img_a">
          <el-input placeholder="请填写图像编号" :maxLength="20" v-model="form.img_a" />
        </el-form-item>
        <el-form-item label="value" prop="value">
          <el-input placeholder="请填写值" :maxLength="20" v-model="form.value" />
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
                  value: -1
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
          if (this.mode === 'add') {
            axios.get(this.$store.getters.getUrl.operation.operation.numberic_calculation.add_between_mat_and_value, {
            params: {
              token: this.$store.getters.getToken,
              img_a: this.form.img_a,
              value: this.form.value,
            }
            }).then(response => {
              if (response.data != null) {
                if (response.data.status === 0) {
                  console.log(response.data)
                  this.result_index = response.data.mat_index
                  mitt.emit('result_index', response.data.mat_index)
                }
              }
            })
        } else if (this.mode === 'sub') {
          axios.get(this.$store.getters.getUrl.operation.operation.numberic_calculation.sub_between_mat_and_value, {
            params: {
              token: this.$store.getters.getToken,
              img_a: this.form.img_a,
              value: this.form.value,
            }
            }).then(response => {
              if (response.data != null) {
                if (response.data.status === 0) {
                  console.log(response.data)
                  this.result_index = response.data.mat_index
                  mitt.emit('result_index', response.data.mat_index)
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