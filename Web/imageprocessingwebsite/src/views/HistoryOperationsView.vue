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
          style="margin: 5%;">
          <el-table-column label="ID" type="index" width="100"></el-table-column>
          <el-table-column label="创建时间" width="100">
            <template v-slot="scope">
              <span>{{ scope.row.time_stamp }}</span>
            </template>
          </el-table-column>
          <el-table-column label="备注"  fixed="right">
            <template v-slot="scope">
              <span>{{ scope.row.note }}</span>
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
export default {
  mounted() {
    this.getHistoryOperations()
  },
  data() {
    return {
      history_operations: ref([])
    }
  },
  methods: {
    getHistoryOperations() {
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
            this.history_operations = this.$store.getters.getUserBaseMsg.value.history_operations.toList()
          }
        }
      })
    }
  },
}
</script>

<style>

</style>