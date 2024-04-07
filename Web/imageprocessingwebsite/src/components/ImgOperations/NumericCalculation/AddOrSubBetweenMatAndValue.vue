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
    </div>
  </template>
  
  <script>
  export default {
      data() {
          return {
              form: {
                  img_a: -1,
                  value: -1,
              },
              rules: [
                  {}
              ]
          }
      }
  }
  </script>
  
  <style>
  
  </style>