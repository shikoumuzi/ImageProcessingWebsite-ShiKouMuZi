<template>
    <div>
        <UsertPwdVue/>
        <el-row :gutter="0">
          <el-col :span="1"></el-col>
          <el-col :span="22">
            <div style="
                  border-width: 1px; 
                  background-color: white; 
                  border-color: rgb(207, 204, 204);
                  border-radius: 12px;
                  border-style: solid;
                  padding: 1%;"
                  >
                <h style="font-size: large; margin: 2%;">
                查看用户数据
                </h>
            </div>
            <el-table
              :data="this.part_of_users"
              :scroll="{ x: 1200 }"
              :loading="loading"
              bordered
              border
              rowKey="id"
              height="770"
              @change="changePage"
              stripe="true"
              style="margin-top: 2%; text-align: center;"
            >
              <el-table-column label="ID" type="index" width="50"></el-table-column>
              <el-table-column label="UserID" prop="user_id" width="100"></el-table-column>
              <el-table-column label="权限" width="100">
                <template v-slot="scope">
                  {{ this.displayAuthority(scope.row.authority) }}
                </template>
              </el-table-column>
              <el-table-column label="历史操作数" prop="history_operation_count" width="100"></el-table-column>
              <el-table-column label="结果图像数" prop="result_image_count" width="100"></el-table-column>
              <el-table-column label="创建时间"  width="150">
                <template v-slot="scope">
                  {{ this.displayCreateTime(scope.row.time_stamp) }}
                </template>
              </el-table-column>
              <el-table-column label="操作"  fixed="right">
                <template v-slot="scope">
                  <el-button type="info" @click="changeUserPwd(scope.row.user_id, scope.row.authority, scope.$index)">修改</el-button>
                  <el-popconfirm
                      width="220"     
                      confirm-button-text="OK"
                      cancel-button-text="No, Thanks"
                      :icon="InfoFilled"
                      icon-color="#626AEF"
                      title="确定删除吗"
                      @confirm="eraseUserMsg(scope.row, scope.row.authority, scope.$index)"
                    >
                    <template #reference>
                        <el-button type="danger">删除</el-button>
                    </template>
                  </el-popconfirm>
                </template>
                
              </el-table-column>
            </el-table>
            <el-pagination
              background
              layout="prev, pager, next,jumper, ->, total"
              :total="users.length"
              @current-change="handleCurrentChange"
              :current-page="current_page"
              :page-count="page_size"
              style="text-align: center">
             </el-pagination>

          </el-col>
          <el-col :span="1"></el-col>
        </el-row>
    </div>
</template>

<script>
import { ElNotification } from 'element-plus';
import axios from '../../plugin/AxiosAPI';
import UsertPwdVue from '../../components/UsertPwd.vue';
import mitt from '../../plugin/MittAPI';
export default {
    name: 'WebUserMsg',
    components: {
      UsertPwdVue
    },
    data() {
        return {
            users: [],
            part_of_users: [],
            current_page: 1,
            page_size: 1
        };
    },

    mounted() {
        // console.log(this.$store.getters.getUrl.manager.user.getAllUserMsg)
        if (this.$store.getters.getManagerStore.value !== null) {
            if (this.$store.getters.getManagerStore.value.user_msg === null || 
            this.$store.getters.getManagerStore.value.user_msg === undefined) {
                axios.post(this.$store.getters.getUrl.manager.user.getAllUserMsg, {
                    params: {
                        token: this.$store.getters.getToken
                    }
                }).then(response => {
                    // console.log('response', response)
                    if (response.data !== null) {
                        if (response.data.status === 0) {
                            this.users = response.data.user_msg
                            this.$store.commit('setThePropertyOfManagerStore', { property_name: 'user_msg', data: response.data.user_msg })
                            // console.log(response.data.user_msgs)
                            this.part_of_users = this.users.slice(0, 15)
                            this.page_size = parseInt((this.users.length / 15))
                        }
                    }
                })
            } else {
              // console.log(this.$store.getters.getManagerStore.value.user_msg)
              this.users = this.$store.getters.getManagerStore.value.user_msg
              this.part_of_users = this.users.slice(0, 15)
              this.page_size = parseInt((this.users.length / 15) + 1) 
            }
        }
    },

    methods: {
        // eslint-disable-next-line camelcase
        displayCreateTime(time_stamp) {
          return new Date(time_stamp).toDateString()
        },
        displayAuthority(authority) {
          // console.log(authority)
          if (authority === 1) {
            return '普通用户'
          } else if (authority === 2) {
            return '管理员'
          }
        },
        // eslint-disable-next-line camelcase
        handleCurrentChange(current_page) {
          // eslint-disable-next-line camelcase
          this.current_page = current_page
          // eslint-disable-next-line camelcase
          this.part_of_users = this.users.slice((current_page - 1) * 15, (current_page) * 15)
        },
        // eslint-disable-next-line camelcase
        changeUserPwd(user_id, authority, index) {
          if (authority === 2) {
            ElNotification.error({
              title: '错误',
              message: '权限不足',
              duration: 4000
            })
            return
          }
          // console.log(user_id)
          mitt.emit('setUserPwdDialog', {
            dialogVisible: true,
            user_id: user_id
          })
        },
        // eslint-disable-next-line camelcase
        eraseUserMsg(index, authority, user_id) {
          if (authority === 2) {
            ElNotification.error({
              title: '错误',
              message: '权限不足',
              duration: 4000
            })
            return
          }

          this.users.splice((this.current_page - 1) * 15 + index, 1)
          this.part_of_users.splice(index, 1)

          axios.post(this.$store.getters.getUrl.manager.user.eraseUserMsg, {
            params: {
              token: this.$store.getters.getToken,
              user_id: user_id
            }
          }).then(response => {
            if (response.data !== null) {
              if (response.data.status === 0) {
                ElNotification.success({
                  title: '成功',
                  message: '删除成功',
                  duration: 4000
                })
                return
              }
            }
            ElNotification.error({
              title: '删除失败',
              message: '删除失败',
              duration: 4000
            })
          })

          if (this.users.length < (this.current_page) * 15) {
            return
          }
          try {
            this.part_of_users.push(this.users[(this.current_page) * 15])
          } catch {
            
          } 
        }
    },
};
</script>

<style lang="scss" scoped>

</style>