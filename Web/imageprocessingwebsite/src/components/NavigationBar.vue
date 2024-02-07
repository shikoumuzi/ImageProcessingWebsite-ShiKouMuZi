<!-- eslint-disable vue/no-deprecated-slot-attribute -->
<template>
    <div>
        <el-menu 
          :default-active="current" 
          mode="horizontal"
          router="true"
          bg-color="#F5F5F5" 
          class="menu">
          <div style="display: flex; align-items: center;">
            <el-menu-item 
              index="1"
              route="/"
              class="menu_item">
                首页
            </el-menu-item>
            <el-menu-item 
              index="2"
              route="/image_operation"
              class="menu_item">
                图像处理
            </el-menu-item>
            <el-menu-item 
              index="3"
              route="/about"
              class="menu_item">
                介绍
            </el-menu-item>
          </div>
          <el-menu-item 
            index="4"
            route="/user"
            class="menu_item">
              用户
              <el-submenu>
                <span slot="title">用户</span>
                <el-menu-item index="1">个人信息</el-menu-item>
                <el-menu-item index="2">登出</el-menu-item>
              </el-submenu>
          </el-menu-item>
          
        </el-menu>
    </div>
</template>

<script>
import { ElMenu } from 'element-plus'
export default {
  components: {
    ElMenu
  }
}
</script>

<style>
.menu_item
{
  margin: 10%;
  margin-left: 2%;
}
.menu
{
  display: flex;
  justify-content: space-between;
  background-color: rgba(238, 238, 212, 0.185);
}
</style>