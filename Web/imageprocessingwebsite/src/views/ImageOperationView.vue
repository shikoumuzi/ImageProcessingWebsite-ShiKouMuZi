<template>
    <el-container>

      <el-container>
        <el-main>
        </el-main>
      </el-container>
      <el-aside width="200px" style="background-color: rgb(252, 250, 238); height: 100%;">
        <request-img-list>
        </request-img-list>
      </el-aside>
    </el-container>

</template>

<script>
import RequestImgList from '@/components/RequestImgList.vue';
export default {
  components: {
    RequestImgList
  },

}
</script>
    RequestImgList

<style>

</style>