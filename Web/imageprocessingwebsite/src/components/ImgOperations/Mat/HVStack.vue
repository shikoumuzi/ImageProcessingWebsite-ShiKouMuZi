<template>
  <div>
    <el-form ref="form" :model="form" :rules="rules" label-width="80px">
      <el-form-item label="姓名" prop="name">
        <el-input placeholder="请填写姓名" :maxLength="20" v-model="form.name" />
      </el-form-item>
      <el-form-item label="性别" prop="sex">
        <el-select placeholder="请选择性别" :options="[{ label: '男', value: '男' }, { label: '女', value: '女' }]" v-model="form.sex" />
      </el-form-item>
    </el-form>
  </div>
</template>

<script>
export default {

}
</script>

<style>

</style>