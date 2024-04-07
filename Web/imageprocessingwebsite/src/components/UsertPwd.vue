<template>
    <el-dialog title="修改用户密码" width="600px" v-model="dialogVisible" @close="onClose" style="border-radius: 10px;">
        <el-form ref="userForm" :model="user_form" :rules="rules" label-width="80px">
          <el-form-item label="UserID" prop="username">
            <el-input  :maxLength="20" v-model="user_form.username" readonly/>
          </el-form-item>
          <el-form-item label="新密码" prop="password" >
            <el-input placeholder="请输入新密码" :maxLength="20" v-model="user_form.password"/>
          </el-form-item>
        </el-form>
        <span class="dialog-footer">
            <el-popconfirm
                width="220"
                confirm-button-text="OK"
                cancel-button-text="No, Thanks"
                :icon="InfoFilled"
                icon-color="#626AEF"
                title="确定取消吗"
                @confirm="handleCancel"
                >
                <template #reference>
                    <el-button size="small" >取 消</el-button>
                </template>
            </el-popconfirm>

            <el-button size="small" type="primary" @click="handleOK" >确 定</el-button>
        </span>
    </el-dialog>
</template>

<script>
import mitt from '../plugin/MittAPI'
import axios from '../plugin/AxiosAPI'
import { ElNotification } from 'element-plus'

export default {
    data() {
        return {
            dialogVisible: false,
            user_form: {
                username: '',
                password: ''
            },
            rules: {
                password: [
                { required: true, message: '请输入密码', trigger: 'blur' },
                { min: 6, max: 20, message: '密码长度在6到20个字符之间', trigger: 'blur' },
                // eslint-disable-next-line object-curly-spacing, no-useless-escape
                {pattern: /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[~!@#$%^&*()_+`\-={}:";'<>?,.\/]).{6,20}$/, message: '密码必须包含数字、字母、特殊字符', trigger: 'blur'}

                ]
            }
            
        }
    },
    mounted() {
        mitt.on('setUserPwdDialog', (res) => {
            this.dialogVisible = res.dialogVisible
            this.user_form.username = res.username
        })
    },
    methods: {
        onClose() {
            this.dialogVisible = false
        },
        handleOK() {
            // console.log('handleOK')
            // eslint-disable-next-line no-useless-escape
            const re = /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[~!@#$%^&*()_+`\-={}:";'<>?,.\/]).{6,20}$/
            if (re.test(this.user_form.password) !== true) {
                console.log('失败')
                this.user_form.password = ''
                return
            }
            axios.get(this.$store.getters.getUrl.user.resetPassword, {
                params: {
                    token: this.$store.getters.getToken,
                    username: this.user_form.username,
                    new_password: this.user_form.password
                }
            }).then((res) => {
                if (res.data !== null) {
                    // console.log(res.data)
                    if (res.data.status === 0) {
                        ElNotification.success({
                            title: '成功',
                            message: '成功修改密码',
                            duration: 4000
                        })
                        this.dialogVisible = false
                        this.user_form.password = ''
                        return
                    }
                }
                ElNotification.error({
                            title: '失败',
                            message: '失败修改密码',
                            duration: 4000
                        })
                this.user_form.password = ''
            })
        },
        handleCancel() {
            this.dialogVisible = false
        },
        
    },
}   
</script>

<style>

</style>