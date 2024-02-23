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
          <el-table-column label="操作" width="100" fixed="right">
            <template v-slot="scope" >
              <div style="display: inline-block;">              
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
import { ElNotification } from 'element-plus'
import axios from '../../plugin/AxiosAPI'

export default {
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
      return time_stamp
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