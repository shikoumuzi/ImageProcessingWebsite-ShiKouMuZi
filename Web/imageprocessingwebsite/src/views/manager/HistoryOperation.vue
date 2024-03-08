<template>
  <div>
    <HistoryOperation/>
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
        <div :key="re_patiner_times">
          <el-table
            title="历史操作"
            :data="part_of_history_operations"
            
            :loading="loading"
            bordered
            border 
            height="770"
            rowKey="ID"
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
                  <el-button type="info" @click="spanOperationOfHistoryOperation(scope.row, scope.$index)">查看</el-button>         
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
      </div>
        <el-pagination
              background
              layout="prev, pager, next,jumper, ->, total"
              :total="this.total_data_size"
              @current-change="handleCurrentChange"
              :current-page="current_page"
              :page-count="page_size"
              
              style="text-align: center; margin-top: 1%;">
        </el-pagination>
      </el-col>
      <el-col :span="3"></el-col>
    </el-row> 
  </div>
</template>

<script>
import { ElNotification } from 'element-plus'
import axios from '../../plugin/AxiosAPI'
import HistoryOpertionsSet from '@/typings/HistoryOperationsSet'
import HistoryOperation from '@/components/HistoryOperation.vue'
import mitt from '../../plugin/MittAPI'  
export default {
  components: {
    HistoryOperation
  },
  mounted() {
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
              this.page_size = parseInt((this.history_operations.length / 15) + 1)
              this.total_data_size = history_operations.size()
              this.part_of_history_operations = this.history_operations.slice(0, 15)
              // console.log(this.$store.getters.getManagerStore.value.history_operations)
            }
          }
        })
      } else {
        this.history_operations = this.$store.getters.getManagerStore.value.history_operations.history_operations
        this.page_size = parseInt((this.history_operations.length / 15) + 1)
      
        this.total_data_size = this.$store.getters.getManagerStore.value.history_operations.size()
        this.part_of_history_operations = this.history_operations.slice(0, 15)
      }
    }
  },
  data() {
    return {
        re_patiner_times: 1,
        part_of_history_operations: null,
        history_operations: [],
        total_data_size: 0,
        current_page: 1,
        page_size: 1
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
      if (index > this.$store.getters.getManagerStore.value.history_operations.size()) {
        return
      }
      this.history_operations.splice(index + (this.current_page - 1) * 15, 1) 
      this.part_of_history_operations.splice(index, 1)
      if (this.history_operations.length < (this.current_page) * 15) {
        return
      }
      try {
        this.part_of_history_operations.push(this.history_operations[(this.current_page) * 15])
      } catch {
        
      }
    },
    handleCurrentChange(currentPage) {
      this.current_page = currentPage
      this.part_of_history_operations = this.history_operations.slice((currentPage - 1) * 15, currentPage * 15)
    },
    getRowKey(row) {
      return row.history_operation_id
    },
    spanOperationOfHistoryOperation(row, index) {
      // console.log(row)
      if (row.operations === null || row.operations === undefined || row.operations.length === 0) {
        axios.post(this.$store.getters.getUrl.operation.getOperationByHistoryOperationID, {
        params: {
          token: this.$store.getters.getToken,
          history_operation_id: row.history_operation_id
        }
        }).then((response) => {
          if (response.data !== null) {
            if (response.data.status === 0) {
              // 存储操作详情存在对应的历史操作当中
              // eslint-disable-next-line camelcase
              const current_index = (this.current_page - 1) * 15 + index
              this.$store.commit('setOperationDetailsToOnceOfHistoryOperationByItsIdForManagerStore', { index: current_index, operation_details: response.data.operation_details })
              this.history_operations[current_index].operations = response.data.operation_details
              mitt.emit('setHistroyOperationDialogVisible', {
                dialogVisible: true,
                operations: this.history_operations[current_index].operations
              })
            }
          }
        })  
      } else {
        // eslint-disable-next-line camelcase
        const current_index = (this.current_page - 1) * 15 + index
        mitt.emit('setHistroyOperationDialogVisible', {
                dialogVisible: true,
                operations: this.history_operations[current_index].operations
              })
      }
    }
  }
}
</script>

<style>

</style>