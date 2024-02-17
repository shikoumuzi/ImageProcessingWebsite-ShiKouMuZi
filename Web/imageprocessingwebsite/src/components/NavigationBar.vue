<!-- eslint-disable vue/no-deprecated-slot-attribute -->
<template>
    <div>
        <el-menu 
          :default-active="this.$route.path" 
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
              route="/history_operation"
              class="menu_item">
                历史操作
            </el-menu-item>
            <el-menu-item 
              index="4"
              route="/about"
              class="menu_item">
                介绍
            </el-menu-item>
            <el-menu-item index="5" v-show="this.$store.getters.getUserBaseMsg.value.authority == 2">
                网站管理
            </el-menu-item>
          </div>
          <el-menu-item 
            index="6"
            route="/user"
            class="menu_item"
            @click="toLogin">
            {{ getUserSubMenuTitle() }}
            <el-sub-menu v-show="this.$store.getters.getUserBaseMsg.value.authority !== 0">
              <!-- <span slot="title">用户</span> -->
              <el-menu-item index="1">个人信息</el-menu-item>
              <el-menu-item index="2" @click="signOut">
               登出
              </el-menu-item>
            </el-sub-menu>
          </el-menu-item>
          
        </el-menu>
    </div>
</template>

<script>
import { ElMenu } from 'element-plus'
import { Store } from 'vuex';
export default {
  components: {
    ElMenu
  },
  data () {
    return {
    }
  },
  created () {
  },
  methods: {
    signOut() {
      this.$store.commit('setUserLoginStatus', false)
      this.$store.commit('clearUserMsg')
    },
    getUserSubMenuTitle() {
      if (this.$store.getters.getUserBaseMsg.value.authority === 0) {
        return '登录'
      } else {
        return this.$store.getters.getUserBaseMsg.value.username
      }
    },
    toLogin() {
      if (this.$store.getters.getUserBaseMsg.value.authority === 0) {
        this.$router.push('/login')
      }
    }
  },
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