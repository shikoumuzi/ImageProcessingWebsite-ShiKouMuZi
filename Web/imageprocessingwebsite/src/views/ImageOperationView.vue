<template>
    <el-container>

      <el-container >
        <el-aside width="250px" style="overflow: hidden;">
          <el-menu :default-active="current" mode="vertical">
                
              <el-sub-menu index="m-1">
                <template #title>
                  <span>MMat 基本图像处理</span>
                  </template>
                  <el-menu-item index="m-1-1">createMat</el-menu-item>
                  <el-menu-item index="m-1-2">copy</el-menu-item>
                  <el-menu-item index="m-1-3">resize</el-menu-item>
                  <el-menu-item index="m-1-4">hstack</el-menu-item>
                  <el-menu-item index="m-1-5">vstack</el-menu-item>
                  <el-menu-item index="m-1-6">split</el-menu-item>
                  <el-menu-item index="m-1-6">merge</el-menu-item>
              </el-sub-menu>
 
              <el-menu-item index="2">
                MNumericCalculation 数值计算
              </el-menu-item>
              <el-menu-item index="3">
                MAffineTransformation 仿射变换
              </el-menu-item>
              <el-menu-item index="4">

              </el-menu-item>
              <el-menu-item index="5">

              </el-menu-item>
            </el-menu>
        </el-aside>
        <el-main>
          <el-row :gutter="0" style="overflow: hidden;">
            <el-col :span="20"></el-col>
            <el-col :span="4">
            </el-col>
          </el-row>
        </el-main>
        <el-aside width="300px" style="overflow: hidden;">
          <div class="result_img_list">
                <div style="
                  border-width: 1px; 
                  background-color: white; 
                  border-color: rgb(207, 204, 204);
                  border-radius: 8px;
                  border-style: solid;
                  padding: 1%;
                  margin-bottom: 2%;"
                  >
                <h style="font-size: large; margin: 2%;">
                  结果图片列表
                </h>
              </div>
                <el-scrollbar style="overflow: hidden;">
                  <div v-for="i in this.result_img_count" :key="i" class="result_img_list_item">
                      <div class="result_img">
                          <el-image :src="getImgSrc(i - 1)" :fit="contain" style="width: 80%; max-width: 80%; border: solid rgb(207, 204, 204) 5px;"/>
                          <div style="  
                              display: flex; 
                              flex-direction: column;; 
                              justify-content: center;
                              margin-right: 10px;
                              margin-left: 5px;">
                              <span style="margin-right: 2pt; margin-bottom: 10px; border: solid rgb(207, 204, 204); background-color: rgb(207, 204, 204); width: 20px;">{{ this.result_img_count - i + 1 }}</span>

                              <div style="border: solid rgb(207, 204, 204); background-color: rgb(207, 204, 204); width: 20px;">
                                  <el-icon><Download /></el-icon>
                              </div>

                          </div>
                      </div>
                  </div>
                </el-scrollbar>
              </div>
        </el-aside>
      </el-container>
    </el-container>

</template>

<script>

export default {
  mounted() {
        // this.result_img_count = ((this.$store.getters.getResultImgList).value.length)
    },
    data () {
        return {
            result_img_count: 50
            
        }
    },
    methods: {
        getImgSrc(index) {
          try {
            return window.URL.createObjectURL(((this.$store.getters.getResultImgList).value)[index].img)
          } catch {
            console.log('getImgSrc is loading error')
          }
        },
        load() {
            this.result_img_count = this.$store.state.user_base_msg.result_image_list.length
        }
    },
    watch: {
        '$store.state.user_base_msg.result_image_list.length'(newValue, oldValue) {
            console.log('work')
            this.result_img_count = newValue
        },
    }

}
</script>

<style scoped>
.result_img_list
{
    border-style: solid;
    border-width: 1px;
    border-color: rgb(207, 204, 204);
    /* border-color: black; */
    border-radius: 10px;
    height: 100vh;
    background-color: rgb(255, 255, 255);
    /* background-color: black; */
}

.result_img_list_item {
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 10px;
}

.result_img {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  background-color: rgba(255,251,240, 0.6);
  border-style: solid;
  border-width: 1px;
  border-color: rgb(207, 204, 204);
  max-height: 150px;
  height: 150px;
  width: 98%;
}

</style>