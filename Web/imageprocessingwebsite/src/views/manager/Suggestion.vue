<!-- eslint-disable vue/multi-word-component-names -->
<template>
  <div>
    <div style="
                  border-width: 1px; 
                  background-color: white; 
                  border-color: rgb(207, 204, 204);
                  border-radius: 12px;
                  border-style: solid;
                  padding: 1%;
                  margin-bottom: 2%;"
                  >
          <h style="font-size: large; margin: 2%;">
          意见反馈
          </h>
        </div>
    <div v-for="(suggestion, index) in suggestions" :key="index" class="suggestion-model">
      
      <el-form ref="suggestionForm" :model="suggestion" :rules="rules" label-width="80px">
        <el-form-item label="用户名" prop="user_name">
          <el-input v-model=" suggestion.user_name" readonly></el-input>
        </el-form-item>
        <el-form-item label="建议ID" prop="user_name">
          <el-input v-model=" suggestion.suggestion_id" readonly></el-input>
        </el-form-item>
        <el-form-item label="内容" prop="content">
          <el-input type="textarea" v-model="suggestion.content" readonly/>
        </el-form-item>

        <el-form-item label="反馈" prop="response">
          <el-input type="textarea" v-model="suggestion.response"/>
        </el-form-item>
      </el-form>
      <div style="display: flex; justify-content: flex-end;">
        <el-button type="primary" @click="submitResponseToSuggestion(suggestion.suggestion_id, suggestion.response, index)">提交反馈意见</el-button>
        <el-button type="danger" @click="eraseSuggestion(suggestion.suggestion_id, index)">忽略该建议</el-button>
      </div>
    </div>
    <div v-if="is_loading" class="suggestion-model">
      <el-form ref="suggestionForm" :model="suggestion_form" :rules="rules" label-width="80px" 
        v-loading="true"
        element-loading-text="Loading..."
        :element-loading-spinner="svg"
        element-loading-svg-view-box="-10, -10, 50, 50"
        element-loading-background="rgba(122, 122, 122, 0.8)">
          <el-form-item label="用户名" prop="user_name">
            <el-input v-model=" suggestion_form.user_name" readonly></el-input>
          </el-form-item>
          <el-form-item label="建议ID" prop="user_name">
            <el-input v-model=" suggestion_form.suggestion_id" readonly></el-input>
          </el-form-item>
          <el-form-item label="内容" prop="content">
            <el-input type="textarea" v-model="suggestion_form.content" readonly/>
          </el-form-item>

          <el-form-item label="反馈" prop="response">
            <el-input type="textarea" v-model="suggestion_form.response"/>
          </el-form-item>
      </el-form>
    </div>
  </div>
</template>

<script>
import { ElNotification } from 'element-plus'
import axios from '../../plugin/AxiosAPI'
import { onBeforeUnmount, onUnmounted } from 'vue'

export default {
  setup() {
    onBeforeUnmount(() => {
      try {
        window.removeEventListener('scroll', this.handleScroll)
      } catch {
        
      }
    })
  },

  mounted() {
    this.is_loading = false
    window.addEventListener('scroll', this.handleScroll, true)
    if (this.$store.getters.getManagerStore.value !== null) {
      if (this.$store.getters.getManagerStore.value.suggestions === null || 
        this.$store.getters.getManagerStore.value.suggestions === undefined) {
        axios.post(this.$store.getters.getUrl.manager.suggestion.getAllSuggestions, {
          params: {
            token: this.$store.getters.getToken,
            now_len: 0
          }
        }).then((response) => {
          if (response.data != null) {
            if (response.data.status === 0) {
              this.suggestions = response.data.suggestions
              // console.log('response suggestions is: ', response.data.suggestions)
              // console.log('suggestion size is: ', this.suggestions.length)
              for (let i = 0; i < this.suggestions.length; ++i) {
                this.suggestions[i].response = ''
              }
              this.$store.commit('setThePropertyOfManagerStore', 
                              // eslint-disable-next-line object-curly-newline
                              { property_name: 'suggestions', 
                              // eslint-disable-next-line object-curly-newline
                              data: response.data.suggestions })
            }
          }
        }) 
      } else {
        this.suggestions = this.$store.getters.getManagerStore.value.suggestions
      // console.log(this.suggestions)
      }
    }
  },
  beforeUnmount() {
    window.removeEventListener('scroll', this.handleScroll, false)
    this.is_loading = false
  },
  data() {
    return {
        svg: `
        <path class="path" d="
          M 30 15
          L 28 17
          M 25.61 25.61
          A 15 15, 0, 0, 1, 15 30
          A 15 15, 0, 1, 1, 27.99 7.5
          L 15 15
        " style="stroke-width: 4px; fill: rgba(0, 0, 0, 0)"/>
      `,
      is_loading: false,
      is_end: false,
      suggestions: [],
      suggestion_form: {
        user_name: '',
        content: '',
        response: '',
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
    submitResponseToSuggestion(suggestion_id, response, index) {
      // console.log(suggestion_id, response)
      if (this.$store.getters.getUserBaseMsg.value.authority === 2) {
        axios.post(this.$store.getters.getUrl.manager.suggestion.submitResponseToSuggestionByID, {
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
              this.eraseSuggestion(suggestion_id, index)
            }
          }
        })
      }
    },
    // eslint-disable-next-line camelcase
    eraseSuggestion(suggestion_id, index) {
      if (this.$store.getters.getUrl.manager !== null) {
        axios.post(this.$store.getters.getUrl.manager.suggestion.ignoreSuggestionByID, 
        {
          params: {
            token: this.$store.getters.getToken,
            suggestion_id: suggestion_id
          }
        }).then(response => {
          if (response.data !== null) {
            if (response.data.status === 0) {
              this.suggestions.splice(index, 1)
            }
          }
        })
      }
    },
    handleScroll(e) {
        if (e.target.scrollTop + e.target.clientHeight >= e.target.scrollHeight - 3) {
          if (this.$route.path.indexOf('/manager/suggestion') === -1) {
            return
          }
          console.log('suggestion')
          // console.log(this.is_loading)
          if (this.is_loading === true) {
            return
          }
          if (this.is_end === true) {
            return
          }
          if (this.suggestions.length >= 100) {
            ElNotification.warning({
              title: '警告',
              message: '已加载超过一百条建议，无法加载更多了',
              duration: 3000
            })
            this.is_loading = false
            this.is_end = true
            return
          }
          this.is_loading = true
          
          setTimeout(
            () => {
              if (!this.is_end) {
              // console.log(e.target.scrollTop + e.target.clientHeight, e.target.scrollHeight)
              // console.log('loading')
              axios.post(this.$store.getters.getUrl.manager.suggestion.getAllSuggestions, {
              params: {
                token: this.$store.getters.getToken,
                now_len: this.suggestions.length
              }
              }).then((response) => {
                if (response.data != null) {
                  if (response.data.status === 0) {
                    if (response.data.suggestions.length === 0) {
                      this.is_end = true
                    }
                    // eslint-disable-next-line camelcase
                    const orgin_len = this.suggestions.length
                    this.suggestions = this.suggestions.concat(response.data.suggestions)
                    // console.log('response suggestions is: ', response.data.suggestions)
                    // console.log('suggestion size is: ', this.suggestions.length)
                    // eslint-disable-next-line camelcase
                    for (let i = orgin_len; i < this.suggestions.length; ++i) {
                      this.suggestions[i].response = ''
                    }
                    this.$store.commit('setThePropertyOfManagerStore', 
                                    // eslint-disable-next-line object-curly-newline
                                    { property_name: 'suggestions', 
                                    // eslint-disable-next-line object-curly-newline
                                    data: this.suggestions })
                    this.is_loading = false
                  } else {
                      ElNotification.error({
                        title: '错误',
                        message: '未获取到新的建议',
                        duration: 3000
                  
                      })
                      this.is_loading = false
                  }
                } else {
                      ElNotification.error({
                        title: '错误',
                        message: '未获取到新的建议',
                        duration: 3000
                      })
                      this.is_loading = false
                }
              }) 
            }
            },
            3000
          ) 
        }
    },

  },
  watch: {
  $route: function() {
    window.removeEventListener('scroll', this.handleScroll)
    this.is_loading = false
  }
},
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