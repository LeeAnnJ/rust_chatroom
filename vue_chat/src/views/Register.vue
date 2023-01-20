<template>
  <div>
    <div class="Register">
      <div class="container-left">
        <div>
          <p class="small">Welcome to</p>
          <p class="big"> Chatroom</p>
        </div>
      </div>
      <div class="container-right">
        <div class="mesBox">
          <div class="warnBox" v-if="showWarn">
            注册失败！<br />
            用户名已存在！
          </div>
          <div class="idBox" v-if="showID">
            注册成功！<br />
            请牢记您的用户名！
            <!-- 您的uID为 {{userId}},请牢记！ -->
          </div>
        </div>        
        <div class="content">
          <div>
            <label for="username" class="iconfont icon-wode"> 用户名</label>
            <input type="text" class="user" id="username" ref="inputUsername" />
          </div>
          <div>
              <label for="password" class="iconfont icon-liulan"> 密码</label>
              <input type="text" class="user" id="password" ref="inputPassword" />
          </div>
          <div class="btnBox">
            <button class="button" @click="registerRoom">注册</button>
          </div>
          <div class="btnBox">
            <button class="button" @click="goLogin">去登录</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
// import Room from './Room'
// import io from 'socket.io-client' //引入socket.io-client
export default {
  name: 'Register',
  // components: { Room },
  data() {
    return {
      showWarn: false,
      showID: false,
      // userId: -1,
      user: {},
      userList: [],
      message: {},
    }
  },
  methods: {
    // handleFile(file) {
    //   this.socket.emit('sendImage', { ...this.user, file })
    // },
    // clickImg(index, item) {
    //   this.currentIndex = index
    //   this.currentImg = item
    // },
    registerRoom() {
      // 1.获取用户名
      const username = this.$refs.inputUsername.value
      const password = this.$refs.inputPassword.value
      if (!username.trim()) {
        alert('请输入用户名')
        return
      }
      if (!password) {
        alert('请输入密码')
        return
      }
      this.$api.userApi.getUserByName({name:username}).then((res)=>{
        // 这里的判断是否写的合理？还是写 if(users)?
        // console.log(res.users.length);
        if(res.users.length > 0){
          this.showWarn = true;
        }
        else{
          this.$api.userApi.register({name:username,password:password}).then((res)=>{
            console.log(res);
            // userId = uid;
            this.showID = true;
          })
        }
      })
      // this.showWarn = warn;
      // console.log(this.showWarn);
      // // 2.需要告诉socket io服务，进行登录
      // this.socket.emit('container', {
      //   username,
      //   avatar: this.currentImg,
      // })
    },
    sendServer(content) {
      const { username, password } = this.user
      this.socket.emit('sendMessage', { msg: content, username, password })
    },
    goLogin(){
      this.$router.push('/Login');
    }
  },
  mounted() {
    /**
     * 聊天室的主要功能
     */
    // 1.连接服务器
    // baseURL:process.env.VUE_APP_URL || "/admin/api",
    // this.socket = io(process.env.VUE_APP_URL || "/")
    // 2.监听登录失败的请求
    // this.socket.on('userExit', (data) => alert(data.msg))
    // 3.监听登录成功的请求
    // this.socket.on('loginsuccess', (data) => {
    //   alert(data.msg)
    //   this.user = data
    //   this.isShow = false
    // })
    // this.socket.on('addUser', (data) => {
    //   this.$store.commit('setJoinRoom', data)
    // })
    // this.socket.on('leaveroom', (data) => {
    //   this.$store.commit('setLeaveRoom', data)
    //   this.$refs.chatroom ? this.$refs.chatroom.haveOneleaveRoom() : null
    // })
    // 监听用户列表的信息
    // this.socket.on('userList', (data) => {
    //   this.userList = data
    // })
    // 监听聊天的消息
    // this.socket.on('receiveMessage', (data) => {
    //   // 把接收到的消息显示到聊天窗口中
    //   this.message = data
    // })
    // 监听图片的消息
    // this.socket.on('receiveImage', (data) => {
    //   // 把接收到的图片显示到聊天窗口中
    //   this.$refs.chatroom.handleImage(data)
    // })
  }
}
</script>

<style lang="scss" scoped>
.Register {
  width: 600px;
  height: 360px;
  margin: 130px auto;
  display: flex;
  .container-left {
    width: 260px;
    height: 100%;
    background-color: rgba(66, 69, 120, 0.76);
    display: flex;
    justify-content: center;
    align-items: center;
    .small {
      color: #f1e9e9;
      font-size: 14px;
      font-family: sans-serif;
    }
    .big {
      font-size: 20px;
      font-weight: 600;
      margin-top: 5px;
      color: #f1e9e9;
      font-family: sans-serif;
    }

  }
  .container-right {
    width: 260px;
    // display: flex;
    justify-content: center;
    // align-items: center;
    background-color: #fff;
    label {
      color: #000;
      font-size: 14px;
    }
    .mesBox{
      margin:10px 10px 0px 10px;
      width: 240px;
      height: 80px;
      text-align: left;
    }
    
    .content {
      margin: 0 10px 0 10px;
      width: 240px;
      height: 180px;
      .user {
        width: 95%;
        border: 1px solid #ccc;
        font-size: 14px;
        line-height: 30px;
        padding-left: 10px;
        display: block;
      }
    }
    .btnBox {
      margin-top: 20px;
      .button {
        width: 100px;
        line-height: 30px;
        background-color: #705a76;
        color: #fff;
        border-radius: 10px;
      }
    }
    
  }
}
</style>
