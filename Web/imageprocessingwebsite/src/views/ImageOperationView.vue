<template>
    <el-container>

      <el-container>
        <el-main>
          <el-row :gutter="0">
            <el-col :span="5"></el-col>
            <el-col :span="14"></el-col>
            <el-col :span="5" style="background-color: rgb(252, 250, 238); height: 100%;">
            </el-col>
          </el-row>
        </el-main>
      </el-container>
    </el-container>

</template>

<script>
import RequestImgList from '@/components/RequestImgList.vue';
export default {
  components: {
    // eslint-disable-next-line vue/no-unused-components
    RequestImgList
  },

}
</script>
    RequestImgList

<style>

</style>