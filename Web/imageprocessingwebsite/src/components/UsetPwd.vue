<template>
    <el-dialog title="title" width="600px" v-model="dialogVisible" @close="onClose">
        <el-form ref="form" :model="form" :rules="rules" label-width="80px">
          <el-form-item label="姓名" prop="name">
            <el-input placeholder="请填写姓名" :maxLength="20" v-model="form.name" />
          </el-form-item>
          <el-form-item label="性别" prop="sex">
            <el-select placeholder="请选择性别" :options="[{ label: '男', value: '男' }, { label: '女', value: '女' }]" v-model="form.sex" />
          </el-form-item>
        </el-form>
        <el-input placeholder="" v-model="value" />
    </el-dialog>
</template>

<script>
import mitt from '../plugin/MittAPI'
export default {
    data() {
        return {
            dialogVisible: false
        }
    },
    methods: {
        onClose() {
            this.dialogVisible = false
        }
    },
}   
</script>

<style>

</style>