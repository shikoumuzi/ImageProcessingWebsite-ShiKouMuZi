<!-- eslint-disable vue/no-parsing-error -->
<template>
  <el-row>
    <el-col :span="8"></el-col>
    <el-col :span="8" style="padding-top: 10px">
      <el-container>
        <el-container>
          <el-main>
              <el-form ref="userForm" :model="user_form" :rules="rules" label-width="80px">
                <el-form-item label="用户名" prop="username">
                  <el-input placeholder="" :maxLength="20" v-model="user_form.username" :readonly="!is_reset"/>
                </el-form-item>
                <el-form-item label="权限级别" prop="authority">
                  <el-input placeholder=""  v-model="user_form.authority" :readonly="!is_reset"/>
                </el-form-item>
                <el-form-item label="创建时间" prop="created_date">
                  <el-input placeholder=""  v-model="user_form.created_date" readonly/>
                </el-form-item>
                <el-form-item label="输入原密码" prop="old_password" v-show="is_reset">
                  <el-input placeholder=""  v-model="user_form.old_password" type="password" show-password/>
                </el-form-item>
                <el-form-item label="输入新密码" prop="new_password" v-show="is_reset">
                  <el-input placeholder=""  v-model="user_form.new_password" type="password" show-password/>
                </el-form-item>
                <el-form-item label="确认密码" prop="check_new_password" v-show="is_reset">
                  <el-input placeholder=""  v-model="user_form.check_new_password" type="password" show-password/>
                </el-form-item>
                <el-form-item >
                  <el-button type="primary" @click="startToChange" v-show="!is_reset">修改密码</el-button>
                  <el-popconfirm
                    width="220"
                    confirm-button-text="OK"
                    cancel-button-text="No, Thanks"
                    :icon="InfoFilled"
                    icon-color="#626AEF"
                    title="确定修改吗"
                    @confirm="submitResetForm"
                  >
                    <template #reference>
                      <el-button type="primary"  v-show="is_reset">确认修改</el-button>
                    </template>
                  </el-popconfirm>
                  <el-button type="info" @click="cancelForChange" v-show="is_reset">取消修改</el-button>
                </el-form-item>
              </el-form>
          </el-main>
        </el-container>
      </el-container>
      </el-col>
    <el-col :span="8"></el-col>
  </el-row>
</template>

<script>
import axios from '../plugin/AxiosAPI'
import { ElNotification } from 'element-plus'
export default {
  mounted() {
    if (this.$store.getters.getUserBaseMsg.value.authority !== 0) {
      this.user_form.username = this.$store.getters.getUserBaseMsg.value.username
      if (this.$store.getters.getUserBaseMsg.value.authority === 1) {
        this.user_form.authority = '普通用户'
      } else if (this.$store.getters.getUserBaseMsg.value.authority === 2) {
        this.user_form.authority = '管理员'
      }
      this.user_form.created_date = new Date(this.$store.getters.getUserBaseMsg.value.time_stamp).toDateString()
    }
  },
  data() {
    const validateOldPass = (rule, value, callback) => {
      if (value === '') {
            callback(new Error('请输入密码'))
          } else if (value.length < 6 || value.length > 20) {
            callback(new Error('密码长度为6-20位'))
          } else {
            console.log(this.$store.getters.getUrl.user.checkPassword)
            axios.post(this.$store.getters.getUrl.user.checkPassword, {
              params: {
                token: this.$store.getters.getToken,
                username: this.$store.getters.getUserBaseMsg.value.username,
                password: value
              }
            }).then((response) => {
              console.log(response)
              if (response.data !== null) {
                if (response.data.status === 0) {
                  callback()
                }
              }
              callback(new Error('输入的密码同原密码不匹配'))
            // eslint-disable-next-line node/handle-callback-err
            }).catch((error) => {
              console.log(error)
              ElNotification.error({
                  title: '错误',
                  message: '检查密码操作失败，服务器未响应',
                  duration: 4000,
                })
                callback(new Error('服务器未响应'))
              })
          }
    }
    const validateNewPass = (rule, value, callback) => {
      if (value === '') {
            callback(new Error('请输入密码'))
          } else if (value.length < 6 || value.length > 20) {
            callback(new Error('密码长度为6-20位'))
          } else {
            if (this.user_form.old_password !== '' && this.user_form.new_password !== this.user_form.old_password) {
              callback()
            } else {
              callback(new Error('新密码不能和原密码一样'))
            }
          }
    }
    const validateCheckPass = (rule, value, callback) => {
      if (value === '') {
            callback(new Error('请输入密码'))
          } else if (value.length < 6 || value.length > 20) {
            callback(new Error('密码长度为6-20位'))
          } else {
            if (this.user_form.new_password !== this.user_form.check_new_password) {
              callback(new Error('输入同上一栏密码不一致'))
            }
          }
    }
    return {
      user_form: {
        username: '',
        authority: '',
        created_date: new Date().toDateString(),
        old_password: '',
        new_password: '',
        check_new_password: ''
      },
      is_reset: false,
      rules: {
        old_password: [
          { required: true, message: '请输入原密码', trigger: 'blur' },
          { min: 6, max: 20, message: '密码长度在6到20个字符之间', trigger: 'blur' },
          // eslint-disable-next-line object-curly-spacing, no-useless-escape
          {pattern: /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[~!@#$%^&*()_+`\-={}:";'<>?,.\/]).{6,20}$/, message: '密码必须包含数字、字母、特殊字符', trigger: 'blur'},
          { validator: validateOldPass, trigger: 'blur' }
        ],
        new_password: [
          { required: true, message: '请输入新密码', trigger: 'blur' },
          { min: 6, max: 20, message: '密码长度在6到20个字符之间', trigger: 'blur' },
          // eslint-disable-next-line object-curly-spacing, no-useless-escape
          {pattern: /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[~!@#$%^&*()_+`\-={}:";'<>?,.\/]).{6,20}$/, message: '密码必须包含数字、字母、特殊字符', trigger: 'blur'},
          { validator: validateNewPass, trigger: 'blur' }
        ],
        check_new_password: [
          { required: true, message: '请再次输入密码', trigger: 'blur' },
          { validator: validateCheckPass, trigger: 'blur' }
        ]
      }
      
    }
  },
  methods: {
    startToChange() {
      this.is_reset = true
    },
    cancelForChange() {
      this.is_reset = false
    },
    submitResetForm() {
      axios.post(this.$store.getters.getUrl.user.resetPassword, {
        params: {
          token: this.$store.getters.getToken,
          username: this.$store.getters.getUserBaseMsg.value.username,
          old_password: this.user_form.old_password,
          new_password: this.user_form.new_password
        }
      }
      ).then((response) => {
        if (response.data !== null) {
          if (response.data.status === 0) {
            ElNotification.success({
              title: '成功',
              message: '修改密码成功',
              duration: 4000
            })
          } else {
            ElNotification.error({
              title: '错误',
              message: '修改密码失败',
              duration: 4000,
            })
          } 
        } 
      })
      // eslint-disable-next-line node/handle-callback-err
      .catch((error) => {
          ElNotification.error({
              title: '错误',
              message: '检查密码操作失败，服务器未响应',
              duration: 4000,
            })
        })
    }
  }
}
</script>

<style>

</style>