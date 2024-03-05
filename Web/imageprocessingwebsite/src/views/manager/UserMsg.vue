<template>
    <div>

        <el-row :gutter="0">
          <el-col :span="3"></el-col>
          <el-col :span="18">
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
              :data="users"
              :scroll="{ x: 1200 }"
              :loading="loading"
              bordered
              rowKey="id"
              height="770"
              @change="changePage"
              style="margin-top: 2%;"
            >
              <el-table-column label="ID" type="index" width="50"></el-table-column>
              <el-table-column label="UserID" prop="user_id" width="100"></el-table-column>
              <el-table-column label="权限">
                <template v-slot="scope">
                  <span>{{ scope.index + 1 }}、{{ scope.row.name }}</span>
                </template>
              </el-table-column>
              <el-table-column label="历史操作数" prop="user_id" width="100"></el-table-column>
              <el-table-column label="结果图像数" prop="user_id" width="100"></el-table-column>
              <el-table-column label="创建时间" prop="user_id" width="100"></el-table-column>
              <el-table-column label="操作" width="100" fixed="right">
                <template v-slot="scope">
                  <el-button type="link" @click="toDo(scope.row)">修改</el-button>
                </template>
              </el-table-column>
            </el-table>
            <el-pagination
              background
              layout="prev, pager, next,jumper, ->, total"
              :total="users.length"
              @current-change="handleCurrentChange"
              :current-page="currentPage"
              :page-size="pageSize"
              style="text-align: center">
             </el-pagination>

          </el-col>
          <el-col :span="3"></el-col>
        </el-row>
    </div>
</template>

<script>
import axios from '../../plugin/AxiosAPI';
export default {
    name: 'WebUserMsg',

    data() {
        return {
            users: []
        };
    },

    mounted() {
        if (this.$store.getters.getManagerStore.value === null) {
            if (this.$store.getters.getManagerStore.value.user_msg === null || 
            this.$store.getters.getManagerStore.value.user_msg === undefined) {
                axios.post(this.$store.getters.getUrl.manager.getAllUserMsg, {
                    params: {
                        token: this.$store.getters.getToken
                    }
                }).then(response => {
                    if (response.data !== null) {
                        if (response.data.status === 0) {
                            this.users = response.data.user_msgs
                        }
                    }
                })
            }
        }
    },

    methods: {
        
    },
};
</script>

<style lang="scss" scoped>

</style>