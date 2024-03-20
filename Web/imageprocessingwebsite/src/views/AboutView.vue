<!-- eslint-disable prefer-const -->
<template>

  <div>
    <el-row :gutter="0">
      <el-col :span="4">
        <el-carousel 
          height="100vh" 
          width="400"
          :autoplay="!is_now_reading" 
          :interval="3000" 
          @change="changeCarousel" 
          type="card"
          direction="vertical"
          style="background-color: rgba(63, 63, 63, 0.5); border-radius: 1%;"
          ref="carousel"
          indicator-position='none'
          @mouseenter="handleMouseEnter"
          @mouseleave="handleMousetLeave"
          @mousewheel.prevent>

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
        <div >
          <el-image :src="require('@/assets/img/about/' + this.src_img_names_list[this.now_carousel_index])" :fit="contain" 
          style="margin-top: 1%; border: 10px solid; border-color: black; border-radius: 10px; max-width: 50% ;"/>
        </div>
        <div style="margin-top: 2%; border: 10px; border-color: rgb(0, 2, 7); background-color: rgba(243, 240, 240, 0.863); border-radius: 10px;"> 
          <div style="padding: 1.2%; border: 10px;">
            <el-collapse v-model="active_names" v-loading >
              <el-collapse-item name="1" title="Introduction" >
                <!--简介-->
                {{ getIntroduction() }}
              </el-collapse-item>
              <el-collapse-item name="2" title="OfficalUrl">
                <!--官网链接-->
                <!-- <a :href="getDownLoadUrl">{{ getDownLoadUrl() }}</a> -->

                  <el-table
                    :data="this.about.offical_url"
                    loading
                    bordered
                    @change="changePage"
                    class="item-table">
                    <el-table-column label='ID' type="index"></el-table-column>
                    <el-table-column label="URL" prop="url" width="350">
                      <template v-slot="scope">
                      <a :href="scope.row.url">{{scope.row.url}}</a>
                    </template>
                    </el-table-column>
                    <el-table-column label="Title" width="100" fixed="right" prop="title"></el-table-column>
                  </el-table>

              </el-collapse-item>
              <el-collapse-item name="3" title="Download">
                <!--下载链接-->
                <!-- <a :href="getDownLoadUrl">{{ getDownLoadUrl() }}</a> -->
                <el-table
                  :data="this.about.download_url"
                  bordered
                  @change="changePage"
                  class="item-table">
                  <el-table-column label='ID' type="index"></el-table-column>
                  <el-table-column label="URL" prop="url" width="350">
                    <template v-slot="scope">
                      <a :href="scope.row.url">{{scope.row.url}}</a>
                    </template>
                  </el-table-column>
                  <el-table-column label="Title" width="100" fixed="right" prop="title"></el-table-column>

                </el-table>
              </el-collapse-item>
              <el-collapse-item name="4" title="RecommendedArticle">
                <!--优秀文章推荐-->
                <!-- <div v-for="(item, index) in getRecommendedArticleUrl()" :key="index">
                  <a :href="item">{{ item }}</a>
                </div> -->
                <el-table
                  :data="this.about.recommended_article_url"
                  bordered
                  @change="changePage"
                  class="item-table">
                  <el-table-column label='ID' type="index"></el-table-column>
                  <el-table-column label="URL" prop="url" width="350">
                    <template v-slot="scope">
                      <a :href="scope.row.url">{{scope.row.url}}</a>
                    </template>
                  </el-table-column>
                  <el-table-column label="Title" width="100" fixed="right" prop="title"></el-table-column>

                </el-table>
              </el-collapse-item>
            </el-collapse>
          </div>
              
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
    window.addEventListener('mousewheel', this.handleScroll, true);
    this.changeCarousel(0)
 },
  data() {
    return {
      now_carousel_index: 0, // 走马灯当前索引位置
      is_mouse_on_slider: false, // 判断鼠标是否在侧边栏
      is_now_reading: false,
      active_names: ref('1'), // 下拉框所用变量
      src_img_names_list: [ // 图像名列表
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
      ], 
      about: { // 单个介绍的基本信息
        introduction: '',
        offical_url: [{ url: '', title: '' }],
        download_url: [{ url: '', title: '' }],
        recommended_article_url: [{ url: '', title: '' }],
      },
    }
  },
  methods: {
    changeCarousel(index) {
      const title = this.src_img_names_list[index].split('.')[0]

      this.about = { // 单个介绍的基本信息
        introduction: '',
        offical_url: [{ url: '', title: '' }],
        download_url: [{ url: '', title: '' }],
        recommended_article_url: [{ url: '', title: '' }],
      }

      this.checkTargetContectTitle(title, (obj) => {
        this.about = {
          introduction: obj.introduction,
          offical_url: obj.offical_url,
          download_url: obj.download_url,
          recommended_article_url: obj.recommended_article_url
        }
      })
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
          if (this.now_carousel_index + 1 > this.src_img_names_list.length - 1) {
              this.now_carousel_index = 0
          } else {
              this.now_carousel_index += 1;
          }
          this.$refs.carousel.setActiveItem(this.now_carousel_index)
      }

      // eslint-disable-next-line eqeqeq
      if (direction === 'up' && e.deltaY <= -100) {
          if (this.now_carousel_index - 1 < 0) {
              this.now_carousel_index = this.src_img_names_list.length - 1;
              // this.setActiveItem(0)
          } else {
              this.now_carousel_index -= 1;
          }
          this.$refs.carousel.setActiveItem(this.now_carousel_index)
      }
    },
    handleMouseEnter(e) {
      this.is_mouse_on_slider = true
      this.is_now_reading = false
    },
    handleMousetLeave(e) {
      if (this.is_mouse_on_slider === true) {
        this.is_now_reading = true
      }
      this.is_mouse_on_slider = false
    },
    // eslint-disable-next-line camelcase
    checkTargetContectTitle(title, callback = (obj) => {}) {
      let result = true
      // console.log(title)
      // console.log(!(title in (this.$store.getters.getAbouts)))
      if (!(title in (this.$store.getters.getAbouts.value))) {
         axios.get(this.$store.getters.getUrl.about, {
          params: {
            token: this.$store.getters.getToken,
            target_content_title: title
          }
        }).then((response) => {
          if (response.data != null) {
            if (response.data.status === 0) {
              this.$store.commit('addAbout', {
                title: title,
                about: {
                  introduction: response.data.introduction,
                  offical_url: response.data.offical_url,
                  download_url: response.data.download_url,
                  recommended_article_url: response.data.recommended_article_url,
                }
              })

              callback((this.$store.getters.getAbouts.value)[title])
              return
            }
            result = false
          }
          result = false
        }).catch((e) => { result = false })
      } else {
        callback((this.$store.getters.getAbouts.value)[title])
      }
      // console.log(result)
      return result
    },
    getIntroduction() {
      return this.about.introduction
    },
    getOfficalUrl() {
      return this.about.offical_url
    },
    getDownLoadUrl() {
      return this.about.download_url
    },
    getRecommendedArticleUrl() {
      return this.about.recommended_article_url
    }

  },
})
</script>

<style scoped lang="css">
::v-deep .el-collapse {
          box-sizing: border-box;
          border: none;
          }
::v-deep .el-collapse-item__header {
  border-bottom: 1px solid #7e8286;
  border-top: 1px;
  background-color: rgb(255, 255, 255);
  text-indent:1em;
  font-size: 20px;
}
/* ::v-deep .el-collapse-item__header.is-active {
  border-bottom: 1px solid #ebeef5;
  background-color: aqua;
  border-radius: 30px;
} */
::v-deep .el-collapse-item__wrap {
  border-top: 1px solid #818c96;
  /* border-bottom: 1px solid #000000; */
  background-color: rgb(255, 254, 248);
  text-align: left;
  text-indent:1em;
}
.item-table {
  text-indent: 0em; 
  font-size: 18px; 
  margin-left: 1%;
  margin-right: 1%;
}

</style>