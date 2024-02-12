<template>

  <div>
    <el-row :gutter="0">
      <el-col :span="4">
        <el-carousel 
          height="100vh" 
          width="400"
          autoplay :interval="4000" 
          @change="changeCarousel" 
          type="card"
          direction="vertical"
          style="background-color: rgba(63, 63, 63, 0.5); border-radius: 1%;"
          ref="carousel"
          @mouseenter="handleMouseEnter"
          @mouseleave="handleMousetLeave">

          <el-carousel-item v-for="(item, index) in src_img_names_list" :key="index">
            <el-image 
              :src="require('@/assets/img/about/' + item)" 
              :fit="contain" 
              @click="chooseDetail"
              name="Sqlite"/>
          </el-carousel-item>

        </el-carousel>
      </el-col>
      <el-col :span="1"></el-col>
      <el-col :span="16">
        <div> 
          <el-collapse v-model="activeNames">
            <el-collapse-item name="1" title="Introduction">
              <!--简介-->
              {{ getIntroduction() }}
            </el-collapse-item>
            <el-collapse-item name="2" title="OfficalUrl">
              <!--官网链接-->
              {{ getDownLoadUrl() }}
            </el-collapse-item>
            <el-collapse-item name="3" title="Download">
              <!--下载链接-->
              {{ getDownLoadUrl() }}
            </el-collapse-item>
            <el-collapse-item name="4" title="RecommendedArticle">
              <!--优秀文章推荐-->
              {{ getRecommendedArticleUrl() }}
            </el-collapse-item>
          </el-collapse>
            
        </div>
      </el-col>
      <el-col :span="3"></el-col>
    </el-row>
  </div>
</template>

<script>
import { ref } from 'vue';
import axios from '../plugin/AxiosAPI';
export default ({
  setup() {
    
  },
  mounted() {
    // 监听鼠标滚动事件
    window.addEventListener('mousewheel', this.handleScroll);
 },
  data() {
    return {
      text_var: '',
      now_carousel_index: 0,
      is_mouse_on_slider: false,
      activeNames: ref('1'),
      src_img_names_list: [
        'OpenCV.png',
        'Rust.jpg',
        'Rocket.png',
        'Cpp.png',
        'C.jpg',
        'CMake.jpg',
        'Web.jpg',
        'Vue.jpeg',
        'Element.png',
        'Sqlite.jpg'
      ]
    }
  },
  methods: {
    chooseDetail() {

    },
    changeCarousel(index) {
      // console.log(index)
    },
    // 判断滚动方向，因为此demo中只有四页，故边界处理为 0 与 3
    handleScroll(e) {
      // e.wheelDellta：可以用来获取鼠标的滚动方向，对于得到的值，只看正负，往上滚是正值，往下滚是负值。
      // 火狐浏览器不支持这个方法，需要会用e.detail来获取滚轮的滚动方向，向上是负值，向下是正值
      const direction = e.deltaY > 0 ? 'down' : 'up'; // deltaY为正则滚轮向下，为负滚轮向上
      // 100为用户一次滚动鼠标的wheelDelta的值,125与鼠标滚动一下是几行的不同而不同
      // eslint-disable-next-line eqeqeq
      if (this.is_mouse_on_slider === false) {
        return
      }

      if (direction === 'down' && e.deltaY >= 100) {
          if (this.now_carousel_index > 9) {
              this.now_carousel_index -= 10
          } else {
              this.now_carousel_index += 1;
          }
          this.$refs.carousel.setActiveItem(this.now_carousel_index)
      }

      // eslint-disable-next-line eqeqeq
      if (direction === 'up' && e.deltaY <= -100) {
          if (this.now_carousel_index < 0) {
              this.now_carousel_index += 10;
              // this.setActiveItem(0)
          } else {
              this.now_carousel_index -= 1;
          }
          this.$refs.carousel.setActiveItem(this.now_carousel_index)
      }
    },
    handleMouseEnter(e) {
      this.is_mouse_on_slider = true
    },
    handleMousetLeave(e) {
      this.is_mouse_on_slider = false
    },
    checkTargetContectTitle(title) {
      if (!(title in this.$store.getters.getAbout.value)) {
        axios.get(this.$store.getters.getUrl.about, {
          params: {
            token: this.$store.getters.getToken,
            target_content_title: title
          }
        }).then((response) => {
          if (response.data != null) {
            if (response.data.status === 0) {
              this.$store.getters.getAbout.value[title] = {
                introduction: response.data.introduction,
                offical_url: response.data.offical_url,
                download_url: response.data.download_url,
                recommended_article_url: response.data.recommended_article_url,
              }
            }
          }
        })
      }
    },
    getIntroduction() {
      // eslint-disable-next-line camelcase
      const now_title = this.src_img_names_list[this.now_carousel_index].split('.')[0]
      // eslint-disable-next-line camelcase
      this.checkTargetContectTitle(now_title)
      return this.$store.getAbout.value[now_title].introduction
    },
    getOfficalUrl() {
      // eslint-disable-next-line camelcase
      const now_title = this.src_img_names_list[this.now_carousel_index].split('.')[0]
      // eslint-disable-next-line camelcase
      this.checkTargetContectTitle(now_title)
      return this.$store.getAbout.value[now_title].offical_url
    },
    getDownLoadUrl() {
      // eslint-disable-next-line camelcase
      const now_title = this.src_img_names_list[this.now_carousel_index].split('.')[0]
      // eslint-disable-next-line camelcase
      this.checkTargetContectTitle(now_title)
      return this.$store.getAbout.value[now_title].download_url
    },
    getRecommendedArticleUrl() {
      // eslint-disable-next-line camelcase
      const now_title = this.src_img_names_list[this.now_carousel_index].split('.')[0]
      // eslint-disable-next-line camelcase
      this.checkTargetContectTitle(now_title)
      return this.$store.getAbout.value[now_title].recommended_article_url
    }

  },
})
</script>
