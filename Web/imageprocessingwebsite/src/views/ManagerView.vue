<template>
  <div>

    <el-container>
      <el-aside width="200px">
        <el-menu :default-active="current" mode="vertical">
            <el-menu-item index="1">
              <router-link to="/manager/suggestion">意见反馈</router-link>
            </el-menu-item>
            <el-menu-item index="2">
              <router-link to="/manager/history_operations">查看历史操作</router-link>
            </el-menu-item>
            <el-menu-item index="3">
              <router-link to="/manager/user_msg">用户管理</router-link>
            </el-menu-item>
            <el-menu-item index="4">
              网页文件查看
            </el-menu-item>
            <el-menu-item index="5">
              查看正在进行的图像操作
            </el-menu-item>
          </el-menu>
      </el-aside>
      <el-container>
        <el-main>
          <el-row :gutter="0">
            <el-col :span="3"></el-col>
            <el-col :span="16">
              <router-view/>
            </el-col>
            <el-col :span="5"></el-col>
          </el-row>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<script>
export default {
  mounted() {
    this.$router.push('/manager/suggestion')
  }
}
</script>

<style>

</style>