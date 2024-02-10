<!-- eslint-disable vue/no-parsing-error -->
<template>
  <el-row>
    <el-col :span="8"></el-col>
    <el-col :span="8" style="padding-top: 10px">
      <el-container>
        <el-container>
          <el-main>
              <el-form ref="userForm" :model="user_form" :rules="rules" label-width="80px">
                <el-form-item label="用户名" prop="user_name">
                  <el-input placeholder="" :maxLength="20" v-model="user_form.user_name" />
                </el-form-item>
                <el-form-item label="权限级别" prop="authority">
                  <el-input placeholder=""  v-model="user_form.authority"/>
                </el-form-item>
                <el-form-item label="创建时间" prop="created_date">
                  <el-input placeholder=""  v-model="user_form.created_date" readonly/>
                </el-form-item>
                <el-form-item label="输入密码" prop="password" v-show="is_reset">
                  <el-input placeholder=""  v-model="user_form.password"/>
                </el-form-item>
                <el-form-item label="确认密码" prop="check_password" v-show="is_reset">
                  <el-input placeholder=""  v-model="user_form.check_password"/>
                </el-form-item>
                <el-form-item >
                  <el-button type="primary" @click="startToChange">修改密码</el-button>
                  <el-button type="primary" @click="cancelForChange" v-show="is_reset">确认修改</el-button>
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
import axios from 'axios'

export default {
  mounted() {
    const usermsg = this.$store.getters.getUserBaseMsg
    if (usermsg.authority !== 0) {
      this.user_form.user_name = usermsg.username
      if (usermsg.authority === 1) {
        this.user_form.authority = '普通用户'
      } else if (usermsg.authority === 2) {
        this.user_form.authority = '管理员'
      }
      this.user_form.created_date = new Date(usermsg.time_stamp).toDateString()
    }
  },
  data() {
    const validatePass = (rule, value, callback) => {
      if (value === '') {
            callback(new Error('请输入密码'))
          } else if (value.length < 6 || value.length > 20) {
            callback(new Error('密码长度为6-20位'))
          } else {
            const usermsg = this.$store.getters.getUserBaseMsg
            axios.post(this.$store.getters.getUrl.checkPassword, {
              params: {
                token: this.$store.getters.getToken,
                username: usermsg.username,
                password: value
              }
            }).then((response) => {
              if (response.data !== null) {
                if (response.data.status === 0) {
                  callback()
                }
              }
              callback(new Error('输入的密码同原密码不匹配'))
            })
          }
    }
    const validateCheckPass = (rule, value, callback) => {
      if (value === '') {
            callback(new Error('请输入密码'))
          } else if (value.length < 6 || value.length > 20) {
            callback(new Error('密码长度为6-20位'))
          } else {
            const usermsg = this.$store.getters.getUserBaseMsg
            axios.post(this.$store.getters.getUrl.resetPassword, {
              params: {
                token: this.$store.getters.getToken,
                username: usermsg.username,
                old_password: this.user_form.password,
                new_password: value
              }
            }).then((response) => {
              if (response.data !== null) {
                if (response.data.status === 0) {
                  callback()
                }
              }
              callback(new Error('修改密码失败'))
            })
          }
    }
    return {
      user_form: {
        user_name: '',
        authority: '',
        created_date: new Date().toDateString(),
        password: '',
        check_password: ''
      },
      is_reset: false,
      rules: {
        password: [
          { required: true, message: '请输入密码', trigger: 'blur' },
          { min: 6, max: 20, message: '密码长度在6到20个字符之间', trigger: 'blur' },
          // eslint-disable-next-line object-curly-spacing, no-useless-escape
          {pattern: /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[~!@#$%^&*()_+`\-={}:";'<>?,.\/]).{6,20}$/, message: '密码必须包含数字、字母、特殊字符', trigger: 'blur'},
          { validator: validatePass, trigger: 'blur' }
        ],
        check_password: [
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
    }
  }
}
</script>

<style>

</style>