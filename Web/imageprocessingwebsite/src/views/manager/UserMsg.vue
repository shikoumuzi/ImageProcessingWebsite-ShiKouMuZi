<template>
    <div>
        <el-row :gutter="0">
          <el-col :span="3"></el-col>
          <el-col :span="18">
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
            
        };
    },

    mounted() {

    },

    methods: {
        
    },
};
</script>

<style lang="scss" scoped>

</style>