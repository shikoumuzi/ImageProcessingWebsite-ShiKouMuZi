<template>
    <div class="  ">
    <el-row>
      <el-col :span="8"></el-col>
      <el-col :span="8" style="padding-top: 0">
        <div style="width:250px;height: 250px" ref="animationContainer"></div>
        <el-container class="align shadow">
          <el-header>
            <span style="font-size: 2em">ImageProcessingWebsite 注册</span>
          </el-header>
          <el-main>
            <el-form :model="register_form" status-icon :rules="rules" ref="registerForm" label-width="100px"
                     class="my-registerForm" label-position="left">

              <el-form-item label="用户名" prop="username">
                <el-input v-model="register_form.username"></el-input>
              </el-form-item>

              <el-form-item label="密码" prop="password">
                <el-input type="password" v-model="register_form.password" autocomplete="off"></el-input>
              </el-form-item>

              <el-form-item label="确认密码" prop="check_password">
                <el-input type="password" v-model="register_form.check_password" autocomplete="off"></el-input>
              </el-form-item>

              <el-form-item>
                <el-button-group class="my-el-button-group">
                  <el-space wrap>
                    <el-button @click="submitForm('registerForm')" class="el-button--primary el-button--large"
                               size="large">
                      <el-text size="large" class="mx-5" style="color: white">注册</el-text>
                    </el-button>
                    <el-button @click="resetForm('registerForm')" class="el-button--large" size="large">
                      <el-text size="large" class="mx-5">重置</el-text>
                    </el-button>
                    <el-button @click="toLogin" text type="primary" class="mx-1" size="large">
                      已有账号？登录！
                    </el-button>
                  </el-space>
                </el-button-group>

              </el-form-item>
            </el-form>
          </el-main>
        </el-container>
      </el-col>
      <el-col :span="8"></el-col>
    </el-row>
  </div>
</template>

<script>
import { ElForm, ElNotification } from 'element-plus'
import axios from '../../plugin/AxiosAPI'
export default {
    name: 'RegisterView',
    components: {

    },
    data () {
        const validateCheckPass = (rule, value, callback) => {
          // console.log(callback)
          if (value === '') {
            callback(new Error('请输入密码'))
          } else if (value.length < 6 || value.length > 20) {
            callback(new Error('密码长度为6-20位'))
          } else {
              if (this.register_form.check_password !== this.register_form.password) {
                callback(new Error('同上一栏密码不相同'))
              } else {
                callback();
              }
          }
        }
        return {
          register_form: {
            username: '',
            password: '',
            check_password: '',
          },
          rules: {
            username: [
              { required: true, message: '请输入用户名', trigger: 'blur' },
              { min: 3, max: 20, message: '用户名长度在3到10个字符之间', trigger: 'blur' },
              { pattern: /^[\u0391-\uFFE5A-Za-z0-9]+$/, message: '请输入不包含特殊字符的中英文以及数字', trigger: 'blur' }
            ],
            password: [
              { required: true, message: '请输入密码', trigger: 'blur' },
              { min: 6, max: 20, message: '用户名长度在6到20个字符之间', trigger: 'blur' },
              // eslint-disable-next-line quotes, no-useless-escape, object-curly-newline
              { pattern: /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[~!@#$%^&*()_+`\-={}:";'<>?,.\/]).{6,20}$/, message: '密码必须包含数字、字母、特殊字符', trigger: 'blur' }
            ],
            check_password: [
              { required: true, message: '请再次输入密码', trigger: 'blur' },
              { validator: validateCheckPass, trigger: 'blur' }
            ]
          }
        }
    },
    methods: {
      toLogin() {
        this.$router.push('/login')
      },
      // eslint-disable-next-line camelcase
      submitForm(register_form_name) {
        this.$ref[register_form_name].validate(
          (valid) => {
            if (valid) {
              axios.get(this.$store.getters.getUrl.user.register, {
                params: {
                  username: this.register_form.username,
                  password: this.register_form.password,
                  time_stamp: new Date().getTime()
                }
              }).then((response) => {
                if (response.data != null) {
                  if (response.data.status === 0) {
                    this.$store.commit('setRegisterName', this.register_form.username)
                    ElNotification.success({
                            title: '成功',
                            message: '注册成功',
                            duration: 4000
                          })
                    this.$router.push('/login')
                  } else if (response.data.status === 1) {
                    ElNotification.error({
                      title: '错误',
                      message: '注册操作失败，当前用户名已存在',
                      duration: 4000,
                    })
                  } else if (response.data.status === 2) {
                    ElNotification.error({
                      title: '错误',
                      message: '注册操作失败，当前用户数已达到最大值',
                      duration: 4000,
                    })
                  }
                }
              // eslint-disable-next-line arrow-spacing, node/handle-callback-err
              }).catch((error)=>{
                ElNotification.error({
                    title: '错误',
                    message: '注册操作失败，服务器未响应',
                    duration: 4000,
                  })
              })
            }
          }
        )
      }
    }
}
</script>

<style>

</style>