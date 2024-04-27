<!-- eslint-disable camelcase -->
<template>
    <el-container>

      <el-container style="height: 100vh;">
        <el-aside width="300px" style="overflow: hidden;">
          <el-menu :default-active="current" mode="vertical">
                
              <el-sub-menu index="m-1">
                <template #title>
                  <span>MMat 基本图像处理</span>
                  </template>
                  <el-menu-item index="m-1-1" @click="changeNowSHowModule('', '')">createMat</el-menu-item>
                  <el-menu-item index="m-1-1" @click="changeNowSHowModule('ReadImg', '')">readImg</el-menu-item>
                  <el-menu-item index="m-1-1" @click="changeNowSHowModule('', '')">saveImg</el-menu-item>
                  <el-menu-item index="m-1-2" @click="changeNowSHowModule('', '')">copy</el-menu-item>
                  <el-menu-item index="m-1-3" @click="changeNowSHowModule('Resize', '')">resize</el-menu-item>
                  <el-menu-item index="m-1-4" @click="changeNowSHowModule('HVStack', 'h')">hstack</el-menu-item>
                  <el-menu-item index="m-1-5" @click="changeNowSHowModule('HVStack', 'v')">vstack</el-menu-item>
                  <el-menu-item index="m-1-6" @click="changeNowSHowModule('')">split</el-menu-item>
                  <el-menu-item index="m-1-6" @click="changeNowSHowModule('')">merge</el-menu-item>
              </el-sub-menu>
              <el-sub-menu index="m-2">
                <template #title>
                  <span>MNumericCalculation 数值计算</span>
                  </template>
                  <el-menu-item index="m-2-1" @click="changeNowSHowModule('AddOrSubBetweenMats', 'add')">addBetweenMats</el-menu-item>
                  <el-menu-item index="m-2-1" @click="changeNowSHowModule('AddOrSubBetweenMatAndValue', 'add')">addBetweenMatAndValue</el-menu-item>
                  <el-menu-item index="m-2-1" @click="changeNowSHowModule('AddOrSubBetweenMatAndScalar', 'add')">addBetweenMatAndScalar</el-menu-item>
                  <el-menu-item index="m-2-2" @click="changeNowSHowModule()">addWeighted</el-menu-item>
                  <el-menu-item index="m-2-3" @click="changeNowSHowModule('AddOrSubBetweenMats', 'sub')">subBetweenMats</el-menu-item>
                  <el-menu-item index="m-2-4" @click="changeNowSHowModule('AddOrSubBetweenMatAndValue', 'sub')">subBetweenMatAndValue</el-menu-item>
                  <el-menu-item index="m-2-5" @click="changeNowSHowModule('AddOrSubBetweenMatAndScalar', 'sub')">subBetweenMatAndScalar</el-menu-item>
                  <el-menu-item index="m-2-6" @click="changeNowSHowModule('Bitwise', 'and')">bitwiseAnd</el-menu-item>
                  <el-menu-item index="m-2-6" @click="changeNowSHowModule('Bitwise', 'or')">bitwiseOr</el-menu-item>
                  <el-menu-item index="m-2-6" @click="changeNowSHowModule('Bitwise', 'not')">bitwiseNot</el-menu-item>
                  <el-menu-item index="m-2-6" @click="changeNowSHowModule('Bitwise', 'xor')">bitwiseXor</el-menu-item>
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
                      
                      <el-image :src="this.now_operation_img_url" :fit="contain" style="height: 100%; width: 100%;"/>
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
                  <div style="margin-top: 2%;"> 
                    <div v-if="this.now_show_method=='ReadImg'">
                      <ReadImg/>
                    </div>
                    <div v-else-if="this.now_show_method=='FreeImg'">
                      <FreeImg/>
                    </div>
                    <div v-else-if="this.now_show_method=='HVStack'">
                      <HVStack/>
                    </div>
                    <div v-else-if="this.now_show_method=='Bitwise'">
                      <Bitwise/>
                    </div>
                    <div v-else-if="this.now_show_method=='Resize'">
                      <Resize/>
                    </div>
                    <div v-else-if="this.now_show_method=='Flip'">
                      <Flip/>
                    </div>
                    <div v-else-if="this.now_show_method=='Rotate'">
                      <Rotate/>
                    </div>
                    <div v-else-if="this.now_show_method=='AddOrSubBetweenMats'">
                      <AddOrSubBetweenMats/>
                    </div>
                    <div v-else-if="this.now_show_method=='AddOrSubBetweenMatAndValue'">
                      <AddOrSubBetweenMatAndValue/>
                    </div>
                    <div v-else-if="this.now_show_method=='AddOrSubBetweenMatAndScalar'">
                      <AddOrSubBetweenMatAndScalar/>
                    </div>
                  </div>
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
                    <div v-for="(item_j, j) in item_i.output_image" :key="item_j" class="result_img_list_item">

                      <div class="result_img">
                          <el-image :src="this.operations_list[i].output_image[j]" :fit="contain" style="width: 80%; max-width: 80%; border: solid rgb(207, 204, 204) 5px;" lazy/>
                          <div style="  
                              display: flex; 
                              flex-direction: column;; 
                              justify-content: center;
                              margin-right: 10px;
                              margin-left: 5px;">
                              <span style="margin-right: 2pt; margin-bottom: 10px; border: solid rgb(207, 204, 204); background-color: rgb(207, 204, 204); width: 20px;">{{ this.operations_list.length - i}}</span>

                              <div style="border: solid rgb(207, 204, 204); background-color: rgb(207, 204, 204); width: 20px;">
                                  <el-icon @click="dowloadResultImg(item_i.mat_index)"><Download /></el-icon>
                              </div>

                          </div>
                          
                      </div>
                      <div>
                        {{ "图片序列号" + " => " + item_i.mat_index }}
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
import AddOrSubBetweenMats from '@/components/ImgOperations/NumericCalculation/AddOrSubBetweenMats.vue'
import AddOrSubBetweenMatAndValue from '@/components/ImgOperations/NumericCalculation/AddOrSubBetweenMatAndValue.vue'
import AddOrSubBetweenMatAndScalar from '@/components/ImgOperations/NumericCalculation/AddOrSubBetweenMatAndScalar.vue'
import ReadImg from '@/components/ImgOperations/Mat/ReadImg.vue'
import FreeImg from '@/components/ImgOperations/Mat/FreeImg.vue'
import HVStack from '@/components/ImgOperations/Mat/HVStack.vue'
import Bitwise from '@/components/ImgOperations/NumericCalculation/Bitwise.vue'
import Resize from '@/components/ImgOperations/Mat/Resize.vue'

import Flip from '@/components/ImgOperations/AffineTransform/Flip.vue'
import Rotate from '@/components/ImgOperations/AffineTransform/Rotate.vue'
import mitt from '../plugin/MittAPI'

import download from 'downloadjs'
import axios from '../plugin/AxiosAPI'

export default {
    components: {
      ReadImg,
      FreeImg,
      HVStack,
      Bitwise,
      Resize,
      Flip,
      Rotate,
      AddOrSubBetweenMats,
      AddOrSubBetweenMatAndValue,
      AddOrSubBetweenMatAndScalar
    },

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
      // for (let i = 0; i < 0; ++i) {
      //   const operation = new Operation()
      //   operation.output_image = [1, 2]
      //   operation.module_name = '123'
      //   operation.method_name = '123'
      //   this.operations_list.push(operation)
      // }
      console.log(this.operations_list)
    },
    created() {
      mitt.on('result_index', (res) => {
        const operation = new Operation()
        // console.log(this.getimg_url + res)
        operation.output_image.push(this.getimg_url + res);
        operation.mat_index = res
        this.operations_list.splice(0, 0, operation);
        this.now_operation_img_url = this.getimg_url + res
        console.log(this.now_operation_img_url)
        console.log(this.operations_list)
      })
    },
    data () {
        return {
            getimg_url: 'http://127.0.0.1:8000/image_processing_website_api/operation/mat/save_img?token=' + this.$store.getters.getToken + '&mat_index=',
            operations_list: [], 
            result_img_count: 0,
            max_operation_count: 20,
            now_show_method: 'ReadImg',
            now_operation_img_url: ''
        }
    },
    beforeUnmount() {
      // 在退出前保存操作
    },
    methods: {
        getImgSrc(i, j) {
          this.result_img_count += 1
          // console.log(this.operations_list)
          console.log(i, j)
          // try {
          //   return ''
          // } catch {
          //   return ''
          // }
          
          // try {
          //   return window.URL.createObjectURL(this.operations_list[i].output_image[j])
          // } catch {
          //   console.log('getImgSrc is loading error')
          // }
        },
        getNowOperatedImg() {
            console.log('getNowOperatedImg')
           return this.getimg_url + this.now_operation_img_index
        },
        async dowloadResultImg(index) {
          // const link = document.createElement('a');
          // link.style.display = 'none';
          // link.href = this.getimg_url + index;
          // document.body.appendChild(link);
          // link.click();
          
          // download(this.getimg_url + index, this.$store.getters.getUserBaseMsg.value.username, 'jpg');
          console.log('download')
          axios.get('/operation/mat/save_img', {
            params: {
              token: this.$store.getters.getToken,
              mat_index: index
            },
            responseType: 'blob',
          }).then((res) => {
            if (res.status === 200) {
              if (res.data !== null) {
                const content = res.data;
                const blob = new Blob([content]);
                if ('download' in document.createElement('a')) {
                // 非IE下载
                const a = document.createElement('a');
                a.download = this.$store.getters.getUserBaseMsg.value.username + index + '.jpg';
                a.style.display = 'none';
                a.href = window.URL.createObjectURL(blob);
                document.body.appendChild(a);
                a.click();
                window.URL.revokeObjectURL(a.href);
                document.body.removeChild(a);
            }
          }
          } 
}
          )
        },
        getServetStatus() {
          return '良好'
        },
        saveToHistoryOperation() {
          
        },
        // eslint-disable-next-line camelcase
        changeNowSHowModule(show_module_name, mode) {
          // eslint-disable-next-line camelcase
          this.now_show_method = show_module_name
          mitt.emit('mode', mode)
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