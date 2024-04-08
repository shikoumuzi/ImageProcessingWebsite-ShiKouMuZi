<!-- eslint-disable vue/no-deprecated-slot-attribute -->
<template>
  <div>
    <el-upload
      class="avatar-uploader"
      action="#"
      :limit="1"
      :show-file-list="false"
      :http-request="handleUpload"
      :before-upload="handleChange"
      accept=".png,.jpe,.jpeg"
      ref="uploadBanner"
      :auto-upload="false"
    >
      <img v-if="formData.appLogo" :src="formData.appLogo" class="avatar" />
      <el-icon v-else class="avatar-uploader-icon"><Plus /></el-icon>
      <el-icon
        v-if="formData.appLogo"
        class="logoDelete"
        @click.stop="clearUploadImg"
        ><CircleCloseFilled
      /></el-icon>
    </el-upload>

  </div>
</template>

<script>

import axios from '../../../plugin/AxiosAPI'
import { ElMessage, genFileId, UploadBanner } from 'element-plus'
import { h, ref } from 'vue';

export default {
    data() {
      return {
         upload: ref(),
         file_list: ref([]),
         uploadBanner: UploadBanner,
         formData: new FormData()
      }
    },
    methods: {
      async uploadFile(fd) {
        console.log(fd)
      },
      handleChange(rawFile) {
        if (rawFile.type !== 'image/jpeg' && rawFile.type !== 'image/png') {
          ElMessage.error('只能上传jpeg/jpg/png图片');
          return false;
        } else if (rawFile.size / 1024 / 1024 > 1) {
          ElMessage.error('上传图片最大不超过1MB!');
          return false;
        }
        return true;
      },
      async handleUpload(file) {
          const fd = new FormData();
          fd.append('token', this.$store.getters.getToken)
          fd.append('file_name', file.name)
          fd.append('file', file.file);
          // 这里是请求上传接口
          const result = await this.uploadFile(fd);
          if (result.code === 200) {
            // // 后台只返回文件名称，需要拼接
            //   this.formData.value.appLogo =
            //   import.meta.env.VITE_APP_HOSTURL +
            //   import.meta.env.VITE_APP_BASEURL +
            //   'file/previewFile/' +
            //   result.data;
            // // 去掉form表单验证的*
            // // formRef.value.clearValidate(['appLogo'])
            // // 上传成功清空文件
            this.uploadBanner.value.handleRemove(file);
          } else {
            this.formData.value.appLogo = '';
            ElMessage.error(result.message);
            this.uploadBanner.value.handleRemove(file);
          }
      },
      clearUploadImg(file) {
          this.formData.value.appLogo = '';
          this.uploadBanner.value.clearFiles();
      }
    }
}
</script>

<style>

</style>