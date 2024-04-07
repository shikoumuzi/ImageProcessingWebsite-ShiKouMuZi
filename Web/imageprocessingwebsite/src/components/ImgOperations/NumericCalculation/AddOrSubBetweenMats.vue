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
  export default {
      data() {
          return {
              form: {
                  img_a: -1,
                  img_b: -1,
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