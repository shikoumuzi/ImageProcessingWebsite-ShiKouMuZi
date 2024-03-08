<template>
    <div class="">
    <el-row>
      <el-col :span="8"></el-col>
      <el-col :span="8" style="padding-top: 5px">
        <div style="width:250px;height: 200px" ref="animationContainer"></div>
        <el-container class="align shadow">
          <el-header>
            <span style="font-size: 2em">ImageProcessingWebsite 登录</span>
          </el-header>
          <el-main>
            <el-form :model="login_form"
                     status-icon
                     :rules="rules"
                     ref="loginForm"
                     label-width="100px"
                     label-position="left">
              <el-form-item label="用户名" prop="username">
                <el-input v-model="login_form.username"></el-input>
              </el-form-item>
              <el-form-item label="密码" prop="password">
                <el-input type="password" v-model="login_form.password" autocomplete="off"></el-input>
              </el-form-item>
              <el-form-item>
                <el-button-group class="my-el-button-group">
                  <el-space wrap>
                    <el-button @click="submitLoginForm" class="el-button--primary el-button--large"
                               size="large">
                      <el-text size="large" class="mx-5" style="color: white">登录</el-text>
                    </el-button>
                    <el-button @click="resetForm" class="el-button--large" size="large">
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
import axios from '../../plugin/AxiosAPI'
import { componentSizeMap, ElNotification } from 'element-plus';

export default {
    name: 'LoginView',
    mounted () {
      if (this.$store.getters.getFrom === '/register') {
        if (this.$store.getters.getRegisterName !== '') {
          this.login_form.username = this.$store.getters.getRegisterName
          this.$store.commit('setRegisterName', '')
        }
      }
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
              { pattern: /^[\u0391-\uFFE5A-Za-z0-9]+$/, message: '请输入不包含特殊字符的中英文以及数字', trigger: 'blur' }
            ],  
            password: [
              { required: true, message: '请输入密码', trigger: 'blur' },
              { min: 6, max: 20, message: '密码长度在6到20个字符之间', trigger: 'blur' },
              // eslint-disable-next-line object-curly-spacing, no-useless-escape
              {pattern: /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[~!@#$%^&*()_+`\-={}:";'<>?,.\/]).{6,20}$/, message: '密码必须包含数字、字母、特殊字符', trigger: 'blur'}

            ]

          }

      };
    },
    methods: {
        // eslint-disable-next-line camelcase
        submitLoginForm() {
          // console.log(this.$refs)
          this.$refs.loginForm.validate(
            (valid) => { 
              if (valid) {
                axios.post(this.$store.getters.getUrl.user.login, {
                  params: {
                    username: this.login_form.username,
                    password: this.login_form.password
                  }
                }).then((response) => {
                  if (response.data !== null) {
                    if (response.data.status === 0) {
                      // 初始化一系列值
                      this.$store.commit('setUserLoginStatus', true)
                      this.$store.commit('setUserName', this.login_form.username)
                      this.$store.commit('setToken', response.data.token)
                      this.$store.commit('setAuthority', response.data.authority)
                      this.$store.commit('setTimeStamp', response.data.time_stamp)

                      if (response.data.authority === 2) {
                        this.$store.commit('initManagerStore')
                      }
                      if (response.data.manager_url !== null) {
                        this.$store.commit('setManagerApiUrl', response.data.manager_url)
                      }
                      
                      ElNotification.success({
                            title: '成功',
                            message: '登录成功',
                            duration: 4000 
                          })

                      this.$router.push(this.$store.getters.getFrom)
                      } else { 
                        if (response.data.status === 1) {
                          ElNotification.error({
                            title: '错误',
                            message: '登录操作失败，用户名或者密码错误',
                            duration: 4000
                          })
                        }
                    }
                  }
                // eslint-disable-next-line node/handle-callback-err
                }).catch((error) => {
                  ElNotification.error({
                    title: '错误',
                    message: '登录操作失败，服务器未响应',
                    duration: 4000,
                  })
                })
              }
            }
          )
        },
        toRegister() {
          this.$router.push('/register')
        },
        // eslint-disable-next-line camelcase
        resetForm() {
          this.login_form = {
            username: '',
            password: '',
            authority: '',
          }
          this.$refs.loginForm.resetForm()
        } 

    }
}
</script>

<style>

</style>