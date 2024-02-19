<template>
  <div>
    <el-row :gutter="0">
      <el-col :span="6"></el-col>
      <el-col :span="12">
        <el-table
          :data="history_operations"
          :loading="loading"
          bordered
          border 
          rowKey="id"
          @change="changePage"
          stripe="true"
          style="margin: 5%;">
          <el-table-column label="ID" type="index" width="50"></el-table-column>
          <el-table-column label="创建时间" width="100">
            <template v-slot="scope">
              <span>{{ this.displayTimeStamp( scope.row.time_stamp) }}</span>
            </template>
          </el-table-column>
          <el-table-column label="备注"  fixed="right" min-width='50'>
            <template v-slot="scope">
              <span>{{ scope.row.note }}</span>
            </template>
          </el-table-column>
          <el-table-column label="操作" width="200" fixed="right">
            <template v-slot="scope" >
              <div style="display: inline-block;">
                <el-popconfirm
                    width="220"
                    confirm-button-text="OK"
                    cancel-button-text="No, Thanks"
                    :icon="InfoFilled"
                    icon-color="#626AEF"
                    title="确定使用这条记录吗"
                    @confirm="useOnceOfHistoryOpearations(scope.row, scope.$index)"
                  >
                    <template #reference>
                      <el-button type="primary">使用</el-button>
                    </template>
                  </el-popconfirm>
                
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
      <el-col :span="6"></el-col>
    </el-row> 
  
  </div>
</template>

<script>
import { ref } from 'vue';
import axios from '../plugin/AxiosAPI';
import { ElNotification } from 'element-plus';

export default {
  mounted() {
    this.initHistoryOperations()
  },
  data() {
    return {
      history_operations: ref([])
    }
  },
  methods: {
    initHistoryOperations() {
      console.log(this.$store.getters.getUserBaseMsg.value.history_operations.isNull())
      if (this.$store.getters.getUserBaseMsg.value.history_operations.isNull()) {
        axios.get(this.$store.getters.getUrl.operation.getHistoryOperationsList, {
          params: {
            token: this.$store.getters.getToken,
            username: this.$store.getters.getUserBaseMsg.value.username
          }
        }).then((response) => {
          console.log(response.data)
          if (response.data != null) {
            if (response.data.status === 0) {
              this.$store.commit('setHistoryOperations', response.data.history_operations)
              this.history_operations = this.$store.getters.getHistoryOperationList
            }
          }
        }) 
      }
      this.history_operations = this.$store.getters.getHistoryOperationList
    },
    // eslint-disable-next-line camelcase
    displayTimeStamp(time_stamp) {
      // eslint-disable-next-line camelcase
      if (time_stamp === 0) {
        return ''
      }
      // eslint-disable-next-line camelcase
      return time_stamp
    },
    eraseHistoryOperation(row, index) {
      if (index > this.$store.getters.getUserBaseMsg.value.history_operations.size()) {
        return
      }
      axios.post(this.$store.getters.getUrl.operation.eraseHistoryOperationList, {
        params: {
          token: this.$store.getters.getToken,
          history_operation_id: this.history_operations[index].history_operation_id
        }
      }).then((response) => {
        if (response.data != null) {
          if (response.data.status === 0) {
            this.history_operations.slice(index, 1)
            this.$store.commit('eraseHistoryOperation', index)
            ElNotification.success({
              title: '成功',
              message: '删除成功',
              duration: 4000
            })
          }
        }
      })
    },
    useOnceOfHistoryOpearations(row, index) {
      // 如果此时的操作界面已经有操作正在加载和运行中，那么就先弹出确认框
      if (this.$store.getters.getTheWorkStatusOfOperationView) {
        return
      }
      // 如果当前历史操作并没有存储操作详情
      if (this.$store.getters.getUserBaseMsg.value.history_operations.getEle(index).value.isNotStoreOperations()) {
        // 获取操作信息
        axios.post(this.$store.getters.getUrl.operation.getOperationByHistoryOperationID, {
        params: {
          token: this.$store.getters.getToken,
          history_operation_id: this.history_operations[index].history_operation_id
        }
        }).then((response) => {
          if (response.data !== null) {
            if (response.data.status === 0) {
              // 存储操作详情存在对应的历史操作当中
              this.$store.commit('setOperationDetailsToOnceOfHistoryOperationByItsId', response.data.operation_details)
              // 开始操作
              this.$router.push({ 
                name: 'image_operation',
                params: {
                  
                }
              })
            }
          }
        })  
      } else {
        // 开始操作
        this.$router.push({ 
                name: 'image_operation',
                params: {

                }
              })
      }
    }
    
  },
}
</script>

<style>

</style>