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
          <el-carousel-item>
            <el-image 
              :src="require('@/assets/img/about/OpenCV.png')" 
              :fit="contain" 
              @click="chooseDetail"
              name="OpenCV"/>
          </el-carousel-item>
          <el-carousel-item>
            <el-image 
              :src="require('@/assets/img/about/Rust.jpg')" 
              :fit="contain" 
              @click="chooseDetail"
              name="Rust"/>
          </el-carousel-item>
          <el-carousel-item>
            <el-image 
              :src="require('@/assets/img/about/Rocket.png')" 
              :fit="contain" 
              @click="chooseDetail"
              name="Rocket"/>
          </el-carousel-item>
          <el-carousel-item>
            <el-image 
              :src="require('@/assets/img/about/Cpp.png')" 
              :fit="contain" 
              @click="chooseDetail"
              name="Cpp"/>
          </el-carousel-item>
          <el-carousel-item>
            <el-image 
              :src="require('@/assets/img/about/C.jpg')" 
              :fit="contain" 
              @click="chooseDetail"
              name="C"/>
          </el-carousel-item>
          <el-carousel-item >
            <el-image 
              :src="require('@/assets/img/about/CMake.jpg')" 
              :fit="contain" 
              @click="chooseDetail"
              name="CMake"/>
          </el-carousel-item>
          <el-carousel-item>
            <el-image 
              :src="require('@/assets/img/about/Web.jpg')" 
              :fit="contain" 
              @click="chooseDetail"
              name="Web"/>
          </el-carousel-item>
          <el-carousel-item>
            <el-image 
              :src="require('@/assets/img/about/Vue.jpeg')" 
              :fit="contain" 
              @click="chooseDetail"
              name="Vue"/>
          </el-carousel-item>
          <el-carousel-item>
            <el-image 
              :src="require('@/assets/img/about/Element.png')" 
              :fit="contain" 
              @click="chooseDetail"
              name="Element"/>
          </el-carousel-item>
          <el-carousel-item>
            <el-image 
              :src="require('@/assets/img/about/Sqlite.jpg')" 
              :fit="contain" 
              @click="chooseDetail"
              name="Sqlite"/>
          </el-carousel-item>
        </el-carousel>
      </el-col>
      <el-col :span="16">

      </el-col>
      <el-col :span="4"></el-col>
    </el-row>
  </div>
</template>

<script>

export default ({
  setup() {
    
  },
  mounted() {
     // 监听鼠标滚动事件
     window.addEventListener('mousewheel', this.handleScroll);
 },
  data() {
    return {
      initialIndex: 0,
      is_mouse_on_slider: false
    }
  },
  methods: {
    chooseDetail() {

    },
    changeCarousel(index) {
      console.log(index)
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
      // console.log(this.$refs)
      // console.log('index: ' + this.initialIndex)
      // console.log('direction:' + direction)
      // console.log('deltaY:' + e.deltaY)
      // // eslint-disable-next-line eqeqeq
      // console.log(direction == 'down' && e.deltaY >= 100)
      // // eslint-disable-next-line eqeqeq
      // console.log(direction == 'up' && e.deltaY <= -125)
      if (direction === 'down' && e.deltaY >= 100) {
          if (this.initialIndex > 9) {
              this.initialIndex -= 10
          } else {
              this.initialIndex += 1;
          }
          this.$refs.carousel.setActiveItem(this.initialIndex)
      }

      // eslint-disable-next-line eqeqeq
      if (direction === 'up' && e.deltaY <= -100) {
          if (this.initialIndex < 0) {
              this.initialIndex += 10;
              // this.setActiveItem(0)
          } else {
              this.initialIndex -= 1;
          }
          this.$refs.carousel.setActiveItem(this.initialIndex)
      }
    },
    handleMouseEnter(e) {
      this.is_mouse_on_slider = true
    },
    handleMousetLeave(e) {
      this.is_mouse_on_slider = false
    }

  },
})
</script>
