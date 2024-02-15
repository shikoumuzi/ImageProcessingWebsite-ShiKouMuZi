<template>
  <div>
    <div style="background-color: rgb(31, 31, 31); height: 62vh;">
      <el-row :gutter="0">
        <el-col :span="4"></el-col>
        <el-col :span="16">
          <div style="margin-top: 5%;">
            <h style="color: rgb(19, 141, 255); font-size: 60px; font-family: 'Poppins', Sans-serif;">Image Processing Website</h>
            <div style="display: flex;  justify-content: center;">
              <el-divider style="max-width: 160vh; "/>
            </div>
            <div style="color: white; text-align: left; text-indent: 2em; font-size: 25px; font-family: 'Poppins', Sans-serif; margin-top: 2%;">
              <p>欢迎来到Image Processing Website，这是一个展示基本图像处理算法的网站。你可以在这里看到不同的算法如何操作图像，了解它们的原理和应用场景。你可以点击上方的算法链接，进入算法页面。或者继续阅读，获取图像处理的概述。</p>
              <p>Image Processing Website是基于Rocket，OpenCV，和Vue构建的，它结合了高效的后端，强大的图像处理库，和美观的前端，为你提供了一个友好和实用的图像处理平台。</p>
              <p>在本网站中，你可以上传你自己的图像，来体验不同的图像处理算法的效果。你可以选择不同的参数，来调整算法的输出。我们希望通过这个网站，你可以学习和享受图像处理的乐趣。</p>
            </div>
          </div>
        </el-col>
        <el-col :span="4"></el-col>
      </el-row>
    </div>
    <div style="margin-top: 2%; font-size: 20px; ">
      <h style="font-size: 30px; font-family: 'Poppins', Sans-serif;">Please contact me if you have any questions</h>
      <div style="display: flex;  justify-content: center;">
        <el-divider style="background-color: rgb(44, 43, 43); max-width: 160vh; "/>
      </div>
      <div style="margin-top: 1%; ">
        <el-row :gutter="0">
          <el-col :span="5"></el-col>
          <el-col :span="4">
            <div style="display: flex; flex-direction: column;">
              <font>Github</font>
              <el-image :src="require('@/assets/img/home/github.png')" :fit="contain" class="el-img"/>
              <!-- <div class="img-wrapper">
                <img src="../assets/img/home/github.png" class="img"/>
              </div> -->
            </div>
          </el-col>
          <el-col :span="1"></el-col>
          <el-col :span="4">
            <div style="display: flex; flex-direction: column;">
              <font>Email</font>
              <el-image :src="require('@/assets/img/home/email.png')" :fit="contain" class="el-img" style="margin-top: 8%;"/>
              <!-- <div class="img-wrapper">
                <img src="../assets/img/home/email.png" class="img" style="transform: translateY(-160%); margin-top: 8%;"/>
              </div> -->
            </div>
          </el-col>
          <el-col :span="1"></el-col>
          <el-col :span="4">
            <div style="display: flex; flex-direction: column;">
              <font>Suggestion</font>
              <!-- <div class="img-wrapper">
                <img src="../assets/img/home/suggestion.png" class="img"/>
              </div> -->
              <el-image :src="require('@/assets/img/home/suggestion.png')" :fit="contain" class="el-img" />
            </div>
          </el-col>
          <el-col :span="5"></el-col>
        </el-row>
    
      </div>
    </div>
  </div>
</template>

<script>
// @ is an alias to /src
export default {

  name: 'HomeView',
  components: {
  }
}
</script>

<style scoped>
.img-wrapper {
    display: flex;
    justify-content: center;
    /* border: 1px solid #ccc; */
    overflow: hidden;
}
.img {
    transform: translateY(-120%);
    filter: drop-shadow(0 110px rgb(12, 156, 255));
    max-width: 25%;
    margin: auto; 
    margin-top: 6%;
}
.el-img{
  max-width: 30%; 
  margin: auto; 
  margin-top: 6%; 
  filter: contrast(150%)
}

</style>
