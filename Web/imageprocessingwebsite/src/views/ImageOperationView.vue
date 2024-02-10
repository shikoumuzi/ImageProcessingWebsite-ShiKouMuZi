<template>
  <div>
    <el-container>
      <el-container>
        <el-main></el-main>
      </el-container>
      <el-aside width="200px">
        <request-img-list>
        </request-img-list>
      </el-aside>

    </el-container>
  </div>
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