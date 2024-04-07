<!-- eslint-disable vue/multi-word-component-names -->
<template>
    <div>
      <el-form ref="form" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="图像A" prop="img_a">
          <el-input placeholder="请填写图像编号" :maxLength="20" v-model="form.img_a" />
        </el-form-item>
        <el-form-item label="w" prop="w">
          <el-input placeholder="请填写 width 值" :maxLength="20" v-model="form.w" />
        </el-form-item>
        <el-form-item label="h" prop="h">
          <el-input placeholder="请填写 height 值" :maxLength="20" v-model="form.h" />
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
                  w: -1,
                  h: -1,
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