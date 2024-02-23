<!-- eslint-disable vue/multi-word-component-names -->
<template>
  <div>
    <div v-for="suggestion in suggestions" :key="suggestion" class="suggestion-model">
      
      <el-form ref="suggestionForm" :model="suggestion" :rules="rules" label-width="80px">
        <el-form-item label="用户名" prop="user_name">
          <el-input v-model=" suggestion.user_name" readonly></el-input>
        </el-form-item>
        <el-form-item label="建议ID" prop="user_name">
          <el-input v-model=" suggestion.suggestion_id" readonly></el-input>
        </el-form-item>
        <el-form-item label="状态" prop="status">
          
        </el-form-item>
        <el-form-item label="内容" prop="content">
          <el-input type="textarea" v-model="suggestion.content" readonly/>
        </el-form-item>

        <el-form-item label="反馈" prop="response">
          <el-input type="textarea" v-model="suggestion.response"/>
        </el-form-item>
      </el-form>
      <div style="display: flex; justify-content: flex-end;">
        <el-button type="primary" @click="submitResponseToSuggestion(suggestion.suggestion_id, suggestion.response)">提交反馈意见</el-button>
        <el-button type="danger" @click="ignoreSuggestion(suggestion.suggestion_id)">忽略该建议</el-button>
      </div>
    </div>
  </div>
</template>

<script>
import { ElNotification } from 'element-plus'
import axios from '../../plugin/AxiosAPI'
export default {
  mounted() {
    if (this.$store.getters.getManagerStore.value !== null) {
      if (this.$store.getters.getManagerStore.value.suggestions === null || 
        this.$store.getters.getManagerStore.value.suggestions === undefined) {
        axios.post(this.$store.getters.getUrl.manager.suggestion.getAllSuggestions, {
          params: {
            token: this.$store.getters.getToken
          }
        }).then((response) => {
          if (response.data != null) {
            if (response.data.status === 0) {
              this.suggestions = response.data.suggestions
              for (let i = 0; i < this.suggestions; ++i) {
                this.suggestions[i].response = ''
              }
              this.$store.commit('setThePropertyOfManagerStore', 
                              // eslint-disable-next-line object-curly-newline
                              { property_name: 'suggestions', 
                              // eslint-disable-next-line object-curly-newline
                              data: response.data.suggestions })
              this.suggestions = response.data.suggestions
            }
          }
        })
      } else {
        this.suggestions = this.$store.getters.getManagerStore.value.suggestions
      // console.log(this.suggestions)
      }
    }
  },
  data() {
    return {
      suggestions: [],
      suggestion_form: {
        user_name: '',
        content: '',
        response: '',
        status: '',
        suggestion_id: ''
      },
      rules: {
        response: [
          {}
        ]
      }
    }
  },

  methods: {
    // eslint-disable-next-line camelcase
    submitResponseToSuggestion(suggestion_id, response) {
      console.log(suggestion_id, response)
      if (this.$store.getUserBaseMsg.value.authority === 2) {
        axios.post(this.$store.getUrl.manager.suggestion.submitResponseToSuggestionByID, {
          params: {
            token: this.$store.getters.getToken,
            suggestion_id: suggestion_id,
            response: response
          }
        }).then((response) => {
          if (response.data !== null) {
            if (response.data.status === 0) {
              ElNotification.success({
                title: '成功',
                message: '提交反馈成功',
                duration: 4000
              })
            }
          }
        })
      }
    },
    // eslint-disable-next-line camelcase
    ignoreSuggestion(suggestion_id) {
      console.log(suggestion_id)
    }
  }
}
</script>

<style>
.suggestion-model {
  background-color: rgb(255, 255, 255);
  padding-top: 1.5vh;
  padding-bottom: 1vh;
  padding-right: 1.5vw;
  border-radius: 12px;
  margin-bottom: 0.5vh;
  border-width: 1px;
  border-style: solid;
  border-color: rgb(207, 204, 204);
}
</style>