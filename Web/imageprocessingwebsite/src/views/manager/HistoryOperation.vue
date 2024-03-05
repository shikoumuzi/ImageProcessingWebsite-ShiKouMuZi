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
          查看历史操作
          </h>
        </div>

        <el-table
          title="历史操作"
          :data="history_operations"
          :loading="loading"
          bordered
          border 
          rowKey="id"
          @change="changePage"
          stripe="true"
          style="margin-top: 5%;">
          <el-table-column label="ID" type="index" width="50"></el-table-column>
          <el-table-column label="创建时间" width="150">
            <template v-slot="scope">
              <span>{{ this.displayTimeStamp( scope.row.time_stamp) }}</span>
            </template>
          </el-table-column>
          <el-table-column label="备注"  fixed="right" min-width='50'>
            <template v-slot="scope">
              <span>{{ scope.row.note }}</span>
            </template>
          </el-table-column>
          <el-table-column label="操作" width="150" fixed="right">
            <template v-slot="scope" >
              <div style="display: flex; justify-content: center;">     
                <el-button type="info">查看</el-button>         
                <el-popconfirm
                    width="220"     
                    confirm-button-text="OK"
                    cancel-button-text="No, Thanks"
                    :icon="InfoFilled"
                    icon-color="#626AEF"
                    title="确定删除吗"
                    @confirm="eraseHistoryOperation(scope.row, scope.$index)"
                  >
                  <template #reference>
                      <el-button type="danger">删除</el-button>
                  </template>
                </el-popconfirm>
              </div>
            </template>
          </el-table-column>
        </el-table>
      </el-col>
      <el-col :span="3"></el-col>
    </el-row> 
  </div>
</template>

<script>
import { ElNotification } from 'element-plus'
import axios from '../../plugin/AxiosAPI'
import HistoryOpertionsSet from '@/typings/HistoryOperationsSet'

export default {
  mounted() {
    console.log('loading')
    if (this.$store.getters.getManagerStore.value !== null) {
      if (this.$store.getters.getManagerStore.value.history_operations === null ||
      this.$store.getters.getManagerStore.value.history_operations === undefined) {
        axios.post(this.$store.getters.getUrl.manager.history_operations.getAllHistoryOperation, {
          params: {
            token: this.$store.getters.getToken
          }
        }).then((response) => {
          if (response.data !== null) {
            if (response.data.status === 0) {
              this.history_operations = response.data.history_operations
              // eslint-disable-next-line camelcase
              const history_operations = new HistoryOpertionsSet()
              for (let i = 0; i < this.history_operations.length; ++i) {
                history_operations.push(this.history_operations[i])
              }
              this.$store.commit('setThePropertyOfManagerStore', { property_name: 'history_operations', data: history_operations })
              console.log(this.$store.getters.getManagerStore.value.history_operations)
            }
          }
        })
      } else {
        this.history_operations = this.$store.getters.getManagerStore.value.history_operations.history_operations
      }
    }
  },
  data() {
    return {
        history_operations: [],
      };
  },
  methods: {
    initHistoryOperations() {
      // console.log(this.$store.getters.getUserBaseMsg.value.history_operations.isNull())
      // eslint-disable-next-line no-empty
      if (this.$store.getters.getManagerStore.value.history_operations.isNull()) {
        
      }
    },
    // eslint-disable-next-line camelcase
    displayTimeStamp(time_stamp) {
      // eslint-disable-next-line camelcase
      if (time_stamp === 0) {
        return ''
      }
      // eslint-disable-next-line camelcase
      return new Date(time_stamp).toDateString()
    },
    eraseHistoryOperation(row, index) {
      // 如果超出范围则返回
      // eslint-disable-next-line no-empty
      if (index > this.$store.getters.getUserBaseMsg.value.history_operations.size()) {
        
      }
    },
  }
}
</script>

<style>

</style>