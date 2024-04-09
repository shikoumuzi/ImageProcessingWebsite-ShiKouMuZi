<!-- eslint-disable vue/multi-word-component-names -->
<template>
    <div>
      <el-form ref="form" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="图像A" prop="img_a">
          <el-input placeholder="请填写图像编号" :maxLength="20" v-model="form.img_a" />
        </el-form-item>
        <el-form-item label="flipcode" prop="flip_code">
          <el-input placeholder="请填写反转参数" :maxLength="20" v-model="form.flip_code" />
        </el-form-item>
      </el-form>
      <el-button type="primary" size="large" @click="submit">提交到服务器</el-button>
    </div>
</template>
  
  <script>
  import mitt from '../../../plugin/MittAPI'
  import axios from '../../../plugin/AxiosAPI'
  export default {
      data() {
          return {
              form: {
                  img_a: -1,
                  flip_code: -100
              },
              rules: [
                  {}
              ],
          }
      },
      methods: {
        submit() {
          axios.get(this.$store.getters.getUrl.operation.affine_transform.flip, {
            params: {
              token: this.$store.getters.getToken,
              mat: this.form.img_a,
              flip_code: this.form.flip_code
            }
          }).then(response => {
            if (response.data != null) {
              if (response.data.status != null) {
                console.log('ok')
              }
            }
          })
        }
      }
  }
  </script>
  
  <style>
  
  </style>