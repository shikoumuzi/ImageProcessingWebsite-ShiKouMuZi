<template>
  <div>
    <el-dialog title="title" width="600px" v-model="dialogVisible" @close="onClose">
      <el-table
        :data="data"
        :scroll="{ x: 1200 }"
        :loading="loading"
        bordered
        rowKey="id"
        :pagination="{
          showSizeChanger: true,
          showTotal: (total) => `共total条数据`,
          pageSize: pageSize,
          page: page,
        }"
        @change="changePage"
      >
        <el-table-column label="id" prop="id" width="100"></el-table-column>
        <el-table-column label="name">
          <template v-slot="scope">
            <span>{{ scope.index + 1 }}、{{ scope.row.name }}</span>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="100" fixed="right">
          <template v-slot="scope">
            <el-button type="link" @click="toDo(scope.row)">修改</el-button>
          </template>
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
        this.dialogVisible = res.dialogVisible
        this.operations = res.operations
        // console.log('on => setDialogVisible with ' + res)
      })
      // console.log('listening')
    },
    onClose() {
      this.dialogVisible = false
    }
  }
};
</script>
