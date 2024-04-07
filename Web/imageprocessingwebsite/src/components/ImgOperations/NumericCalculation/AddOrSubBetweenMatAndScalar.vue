<template>
    <div>
      <el-form ref="form" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="图像A" prop="img_a">
          <el-input placeholder="请填写图像编号" :maxLength="20" v-model="form.img_a" />
        </el-form-item>
        <el-form-item label="r" prop="r">
          <el-input placeholder="请填写 r 值" :maxLength="20" v-model="form.r" />
        </el-form-item>
        <el-form-item label="g" prop="g">
          <el-input placeholder="请填写 g 值" :maxLength="20" v-model="form.g" />
        </el-form-item>
        <el-form-item label="b" prop="b">
          <el-input placeholder="请填写 b 值" :maxLength="20" v-model="form.b" />
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
                  r: -1,
                  g: -1,
                  b: -1
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