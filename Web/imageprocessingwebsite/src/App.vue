<template>
  <el-container style="height: 100%; height: 100vh;">
    <el-container>
      <el-header>    
        <NavigationBar>
        </NavigationBar>
      </el-header>
      <el-main style="background-color: rgba(255,251,240, 0.6); height: 100%; padding: 0; margin: 0;">
        <router-view/>
      </el-main>
      <!-- <el-footer>footer</el-footer> -->
    </el-container>
  </el-container>
  
</template>

<script>
import NavigationBar from './components/NavigationBar.vue';

export default ({
  components: {
    NavigationBar
  },

  setup() {
    
  },
})
</script>

<style lang="less">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  height: 100%;
}

nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: #2c3e50;

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}
</style>
