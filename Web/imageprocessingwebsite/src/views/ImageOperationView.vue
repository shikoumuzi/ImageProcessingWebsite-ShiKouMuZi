<template>
    <el-container>

      <el-container style="height: 100vh;">
        <el-aside width="300px" style="overflow: hidden;">
          <el-menu :default-active="current" mode="vertical">
                
              <el-sub-menu index="m-1">
                <template #title>
                  <span>MMat 基本图像处理</span>
                  </template>
                  <el-menu-item index="m-1-1">createMat</el-menu-item>
                  <el-menu-item index="m-1-1">readImg</el-menu-item>
                  <el-menu-item index="m-1-1">saveImg</el-menu-item>
                  <el-menu-item index="m-1-2">copy</el-menu-item>
                  <el-menu-item index="m-1-3">resize</el-menu-item>
                  <el-menu-item index="m-1-4">hstack</el-menu-item>
                  <el-menu-item index="m-1-5">vstack</el-menu-item>
                  <el-menu-item index="m-1-6">split</el-menu-item>
                  <el-menu-item index="m-1-6">merge</el-menu-item>
              </el-sub-menu>
              <el-sub-menu index="m-2">
                <template #title>
                  <span>MNumericCalculation 数值计算</span>
                  </template>
                  <el-menu-item index="m-2-1">addBetweenMats</el-menu-item>
                  <el-menu-item index="m-2-1">addBetweenMatAndValue</el-menu-item>
                  <el-menu-item index="m-2-1">addBetweenMatAndScalar</el-menu-item>
                  <el-menu-item index="m-2-2">addWeighted</el-menu-item>
                  <el-menu-item index="m-2-3">subBetweenMats</el-menu-item>
                  <el-menu-item index="m-2-4">subBetweenMatAndValue</el-menu-item>
                  <el-menu-item index="m-2-5">subBetweenMatAndScalar</el-menu-item>
                  <el-menu-item index="m-2-6">bitwiseAnd</el-menu-item>
                  <el-menu-item index="m-2-6">bitwiseOr</el-menu-item>
                  <el-menu-item index="m-2-6">bitwiseNo</el-menu-item>
                  <el-menu-item index="m-2-6">bitwiseXor</el-menu-item>
              </el-sub-menu>
 
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
          <div style="margin-bottom: 50px;">
            <el-row :gutter="0" >
              <el-col :span="1"></el-col>
              <el-col :span="13">
                <el-col :span="1"></el-col>
                  <el-col :span="23">
                    <div style="height: 50vh; width: 100%; margin-top: 100px; background-color: aqua;">
                      
                      <el-image :src="getNowOperatedImg" :fit="contain" style="height: 100%; width: 100%;"/>
                      <div style="margin-top: 5px;">
                        图像展示区

                      </div>
                    </div>
                  </el-col>
                <!-- <el-col :span="1"></el-col> -->
              
              </el-col>
              <el-col :span="1">
              </el-col>
              <el-col :span="9">
                <div style="
                  border: solid; 
                  border-width: 1px; 
                  border-color: rgb(207, 204, 204); 
                  background-color: white; 
                  border-radius: 10px; 
                  height: 100%; 
                  margin-top: 70px;
                  margin-bottom: 50px;">
                  方法参数填写区

                </div>
              </el-col>
            </el-row>
          </div>
          <div>
            <el-row :gutter="0">
              <el-col :span="1"></el-col>
              <el-col :span="23">
                <div style="
                  width: 100%; 
                  height: 150px; 
                  background-color: white; 
                  margin-top: 8%; 
                  border-radius: 10px; 
                  border: solid; 
                  border-width: 1px; 
                  border-color: rgb(207, 204, 204);
                  padding: 8px;">
                    <el-descriptions title="操作状态" border>
                      <el-descriptions-item label="已进行的操作数量/最大操作数">{{ this.operations_list.length }} / {{ this.max_operation_count }}</el-descriptions-item>
                      <el-descriptions-item label="当前结果图像数量">{{ this.result_img_count }}</el-descriptions-item>
                      <el-descriptions-item label="当前服务器状态">{{ getServetStatus() }}</el-descriptions-item>
                    </el-descriptions>
                    <div style="display: flex; justify-content: end; margin-top: 10px;">
                      <el-button type="primary" @click="saveToHistoryOperation">保存到历史操作记录当中</el-button>
                    </div>
                  </div>
              </el-col>
            </el-row>

          </div>
        </el-main>
        <el-aside width="350px" style="overflow: hidden;">
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
                <el-scrollbar style="overflow: hidden; height: 100vh">
                  <div v-for="(item_i, i) in this.operations_list" :key="item_i" >
                    <div v-for="(item_j, j) in item_i.output_image.length" :key="item_j" class="result_img_list_item">

                      <div class="result_img">
                          <el-image :src="getImgSrc(this.operations_list.length - i, j)" :fit="contain" style="width: 80%; max-width: 80%; border: solid rgb(207, 204, 204) 5px;" lazy/>
                          <div style="  
                              display: flex; 
                              flex-direction: column;; 
                              justify-content: center;
                              margin-right: 10px;
                              margin-left: 5px;">
                              <span style="margin-right: 2pt; margin-bottom: 10px; border: solid rgb(207, 204, 204); background-color: rgb(207, 204, 204); width: 20px;">{{ this.operations_list.length - i}}</span>

                              <div style="border: solid rgb(207, 204, 204); background-color: rgb(207, 204, 204); width: 20px;">
                                  <el-icon @click="dowloadResultImg(item_j)"><Download /></el-icon>
                              </div>

                          </div>
                          
                      </div>
                      <div>
                        {{ "操作方法名：" + item_i.module_name + " => " + item_i.method_name }}
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
import Operation from '@/typings/Operation'

export default {
    // created() {
    //   for (let i = 0; i < 50; ++i) {
    //     const operation = new Operation()
    //     operation.output_image = [1, 2]
    //     this.operations_list.push(operation)
    //   }
    // },
    mounted() {
      // 在进入前先检查有无缓存操作或者路由有无给进来
          // this.result_img_count = ((this.$store.getters.getResultImgList).value.length)
      for (let i = 0; i < 50; ++i) {
        const operation = new Operation()
        operation.output_image = [1, 2]
        operation.module_name = '123'
        operation.method_name = '123'
        this.operations_list.push(operation)
      }
      console.log(this.operations_list)
    },
    data () {
        return {
            operations_list: [], 
            result_img_count: 0,
            max_operation_count: 20
        }
    },
    beforeUnmount() {
      // 在退出前保存操作
    },
    methods: {
        getImgSrc(i, j) {
          this.result_img_count += 1
          try {
            return window.URL.createObjectURL(this.operations_list[i].output_image[j])
          } catch {
            console.log('getImgSrc is loading error')
          }
        },
        getNowOperatedImg() {
           
        },
        dowloadResultImg(img) {
          
        },
        getServetStatus() {
          return '良好'
        },
        saveToHistoryOperation() {
          
        }
    },

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
  flex-direction: column;
  align-content: center;
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