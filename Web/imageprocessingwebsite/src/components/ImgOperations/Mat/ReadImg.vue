<!-- eslint-disable vue/no-deprecated-slot-attribute -->

<template>
  <div>

    <el-upload
      :ref="upload"
      class="upload-demo"
      action="http://127.0.0.1:8000/image_processing_website_api/operation/mat/read_img" 
      :data="formData"
      :on-success="handleSuccess"
      :before-upload="beforeUpload"
      :on-exceed="handleExceed"
      :limit="3"
      :on-remove="handleRemove"
      :file-list="fileList"
      :auto-upload="true"
      :on-error="handleError"
      :show-file-list="true"
      :http-request="submit"
      :on-change="handleChange">
      <el-button size="large" type="primary">点击上传</el-button>
      <div slot="tip" class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div>
    </el-upload>
    <el-button type="primary" size="large" @click="submit">提交到服务器</el-button>
  </div>
</template>

<script>
import axios from '../../../plugin/AxiosAPI'
import ref from 'vue'
import { componentSizeMap } from 'element-plus'
import mitt from '../../../plugin/MittAPI'
export default {
  data() {
    return {
      token: '',
      formData: {
        file_name: '',
        file: ''
      },
      fileList: [],

    }
  },
  methods: {
    submit(data) {
      console.log('this is submit')
      console.log(data)
      const formData = new FormData()
      formData.append('token', this.$store.getters.getToken)
      formData.append('file_name', data.file.name)
      formData.append('file', data.file)
      axios.post(this.$store.getters.getUrl.operation.operation.mat.read_img, 
                    formData,
                  {
                    headers: {
                    'Content-Type': 'multipart/form-data'
                    }
                  }).then((response) => {
        console.log(response.data)
        if (response.data !== null) {
          if (response.data.status === 0) {
            mitt.emit('result_index', response.data.mat_index)
          }
        }
      })
    },
    beforeUpload(file) {
      // console.log(file.name)
      if (file.name.indexOf('jpg') === -1 && file.name.indexOf('png')) {
        return false
      }
      return true
    },
    handleSuccess(response, file, fileList) {
    },
    // eslint-disable-next-line node/handle-callback-err
    handleError(error, uploadFile) {
    },
    handleExceed(files, fileList) {
      // 文件超出个数限制的回调函数
      this.$message.warning(`当前限制选择 ${this.limit} 个文件，本次选择了 ${files.length} 个文件，共选择了 ${files.length + fileList.length} 个文件`)
    },
    handleRemove(file, fileList) {
      // 文件移除的回调函数
      // console.log(file, fileList)
      this.fileList = []
      this.formData = {
        file_name: '',
        file: ''
      }
    },
    handleChange(file, fileList) {
      if (fileList.length > 0) {
        this.fileList = [file]// 这一步，是 展示最后一次选择文件
      }
    }
  }
}
</script>