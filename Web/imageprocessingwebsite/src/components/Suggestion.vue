<!-- eslint-disable vue/multi-word-component-names -->
<!-- eslint-disable vue/no-deprecated-slot-attribute -->
<template>
  <div>
    <el-dialog title="建议提交栏" width="600px" v-model="dialogVisible" @close="onClose" style="border-radius: 10px;">
      <el-form ref="suggestionForm" :model="suggestion_form" :rules="rules" label-width="80px">
      <el-form-item label="内容：" prop="content">
        <el-input placeholder="请输入意见内容" v-model="suggestion_form.content" type="textarea" rows="6"/>
      </el-form-item>
    </el-form> 
      <span slot="footer" class="dialog-footer">
        <el-button size="small" @click="handleCancel">取 消</el-button>
        <el-button size="small" type="primary" @click="handleOk" >确 定</el-button>
      </span>
    </el-dialog>

  </div>
</template>

<script>
import axios from '../plugin/AxiosAPI'
import { ElNotification } from 'element-plus'
import mitt from '../plugin/MittAPI'
export default { 
  data() {
    return {
      // eslint-disable-next-line prefer-regex-literals
      regex: new RegExp('^[\u0391-\uFFE5A-Za-z0-9]+$'),

      suggestion_form: {
        content: '',
        time_stamp: new Date().valueOf()
      },
      rules: {
        content: [
          { required: true, message: '请输入意见内容', trigger: 'blur' },
          { min: 10, max: 1000, message: '意见内容长度在10到1000个字符之间', trigger: 'blur' },
          { pattern: /^[\u0391-\uFFE5A-Za-z0-9]+$/, message: '请输入不包含特殊字符的中英文以及数字', trigger: 'blur' }
        ]
      },
      dialogVisible: false

    }
  },
  mounted() {
    this.listenDialogVisible()
  },
  methods: {
    footer() {

    },
    handleOk() {
      if (!this.regex.test(this.suggestion_form.content) || this.suggestion_form.content.length < 10 || this.suggestion_form.content.length > 1000) {
        ElNotification.error({
                    title: '失败',
                    message: '填写内容不符合基本要求',
                    duration: 4000,
                  })
        return
      }
      // console.log('handle ok')
      axios.get(this.$store.getters.getUrl.suggestion, {
        params: {
          content: this.suggestion_form.content,
          time_stamp: this.suggestion_form.time_stamp,
          token: this.$store.getters.getToken
        }
      }).then((response) => {
        if (response.data !== null) {
          if (response.data.status === 0) {
            ElNotification.success({
                    title: '成功',
                    message: '感谢您的建议！',
                    duration: 4000,
                  })
            this.dialogVisible = false
          } else {
            ElNotification.error({
                    title: '失败',
                    message: '提交建议失败',
                    duration: 4000,
                  })
          }
        }
      // eslint-disable-next-line node/handle-callback-err
      }).catch((error) => {
        ElNotification.error({
                    title: '失败',
                    message: '提交建议失败',
                    duration: 4000,
                  })
      })
    },
    handleCancel() {
      this.dialogVisible = false
    },
    listenDialogVisible() {
      mitt.on('setDialogVisible', (res) => {
        this.dialogVisible = res
        // console.log('on => setDialogVisible with ' + res)
      })
      // console.log('listening')
    }
  }
}
</script>

<style>

</style>