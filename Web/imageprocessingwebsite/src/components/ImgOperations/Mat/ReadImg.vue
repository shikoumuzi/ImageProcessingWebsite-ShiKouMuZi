<!-- eslint-disable vue/no-deprecated-slot-attribute -->
<template>
  <div>
    <el-form ref="form" :model="form" :rules="rules" label-width="80px">
      <el-form-item label="姓名" prop="name">
        <el-upload
          name="file"
          multiple
          :file-list="fileList"
          :action="uploadUrl"
          :on-preview="preview"
          :on-remove="remove"
          :on-exceed="handleExceed"
          :before-remove="beforeRemove"
          :limit="1"
        >
          <el-button size="small" type="primary" @click="upload">点击上传</el-button>
          <div slot="tip" class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div>
        </el-upload>
      </el-form-item>
    </el-form>
  </div>
</template>

<script>
export default {
    methods: {
        upload() {
            
        }
    }
}
</script>

<style>

</style>