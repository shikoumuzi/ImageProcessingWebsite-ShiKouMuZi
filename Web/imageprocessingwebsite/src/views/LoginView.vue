<template>
    <div class="">
    <el-row>
      <el-col :span="8"></el-col>
      <el-col :span="8" style="padding-top: 10px">
        <div style="width:250px;height: 250px" ref="animationContainer"></div>
        <el-container class="align shadow">
          <el-header>
            <span style="font-size: 2em">askLaw登录</span>
          </el-header>
          <el-main>
            <el-form :model="loginForm"
                     status-icon
                     :rules="rules"
                     ref="loginForm"
                     label-width="100px"
                     label-position="left">
              <el-form-item label="用户名" prop="username">
                <el-input v-model="loginForm.username"></el-input>
              </el-form-item>
              <el-form-item label="密码" prop="pass">
                <el-input type="password" v-model="loginForm.pass" autocomplete="off"></el-input>
              </el-form-item>
              <el-form-item>
                <el-button-group class="my-el-button-group">
                  <el-space wrap>
                    <el-button @click="submitForm('loginForm')" class="el-button--primary el-button--large"
                               size="large">
                      <el-text size="large" class="mx-5" style="color: white">登录</el-text>
                    </el-button>
                    <el-button @click="resetForm('loginForm')" class="el-button--large" size="large">
                      <el-text size="large" class="mx-5">重置</el-text>
                    </el-button>
                    <el-button @click="toRegister" text type="primary" class="mx-1" size="large">
                      没有账号？注册！
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
import axios from 'axios'
import { ElNotification } from 'element-plus';

export default {
    name: 'LoginView',
    mounted () {

    },
    data () {
      return {
          login_form: {
            username: '',
            password: '',
            authority: '',
          },
          rules: {
            username: [
              { required: true, message: '请输入用户名', trigger: 'blur' },
              { min: 3, max: 20, message: '用户名长度在3到10个字符之间', trigger: 'blur' },
            ],
            password: [
              { required: true, message: '请输入密码', trigger: 'blur' },
              { min: 6, max: 20, message: '用户名长度在6到10个字符之间', trigger: 'blur' }
            ]
          }

      };
    },
    methods: {
        // eslint-disable-next-line camelcase
        submitLoginForm(login_form_name) {
          this.$ref[login_form_name].validate(
            (valid) => {
              if (valid) {
                axios.post('/api/account/login', {
                  params: {
                    username: this.login_form.username,
                    password: this.login_form.password
                  }
                }).then((response) => {
                  if (response.data !== null) {
                    if (response.data.status === 0) {
                      this.$store.commit('setUserLoginStatus', true)
                      } else { 
                        ElNotification.error({
                        title: '错误',
                        message: response.data.error_msg,
                        showClose: false,
                    })
                    }
                  }
                })
              }
            }
          )
        },
        toReigster() {
          this.$router.push('/reigster')
        }
    }
}
</script>

<style>

</style>