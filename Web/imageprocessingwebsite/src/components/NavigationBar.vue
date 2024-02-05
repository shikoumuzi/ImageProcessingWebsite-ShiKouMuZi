<template>
    <div>
        <el-menu 
          :default-active="current" 
          mode="horizontal"
          router="true">

          <el-menu-item 
            index="1"
            route="/">首页</el-menu-item>
          <el-menu-item 
            index="2"
            route="/image_operation">图像处理</el-menu-item>
          <el-menu-item 
            index="3"
            route="/about">介绍</el-menu-item>
          <el-menu-item 
            index="4"
            route="/user">用户</el-menu-item>
          
        </el-menu>
    </div>
</template>

<script>
import { ElMenu } from 'element-plus'
export default {

}
</script>

<style>

</style>