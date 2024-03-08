<template>
  <div>
    <el-dialog title="查看详情" width="400px" v-model="dialogVisible" @close="onClose" style="border-radius: 10px;">
      <el-table
        :data="operations"
        :scroll="{ x: 1200 }"
        :loading="loading"
        bordered
        rowKey="id"
        @change="changePage"
      >
        <el-table-column label="id" type="index" width="50"></el-table-column>
        <el-table-column label="模块名" prop="module_name">
        </el-table-column>
        <el-table-column label="方法名" prop="method_name" width="100" fixed="right">
        </el-table-column>
      </el-table>
    </el-dialog>
  </div>
</template>

<script>
import mitt from '../plugin/MittAPI';
export default {
  mounted() {
    this.listenDialogVisible()
  },

  data() {
    return {
        operations: [],
        dialogVisible: false
    }
  },
  
  methods: {
    listenDialogVisible() {
      mitt.on('setHistroyOperationDialogVisible', (res) => {
        // console.log(res.dialogVisible)
        this.dialogVisible = res.dialogVisible
        this.operations = res.operations
        // console.log('hist', res.operations)    
        // console.log('on => setDialogVisible with ' + res)
      })
      // console.log('dialogVisible', this.dialogVisible)

      // console.log('listening')
    },

  }
};
</script>
