<template>
  <div>
    <div class="Login" v-if="isShow">
      <div class="container-left">
        <div>
          <p class="small">Welcome to</p>
          <p class="big"> Chatroom</p>
        </div>
      </div>
      <div class="container-right">
        <div class="mesBox">
          <div class="warnBox" v-if="showWarn">
            登录失败！<br />
            请检查用户名及密码是否正确！
          </div>
        </div>
        <div class="content">
          <div>
            <label for="username" class="iconfont icon-wode"> 用户名</label>
            <input type="text" class="user" id="username" ref="inputUsername" />
          </div>
          <div>
              <label for="password" class="iconfont icon-liulan"> 密码</label>
              <!-- todo 记得最后改为 password -->
              <input type="text" class="user" id="password" ref="inputPassword" />
          </div>
          <div class="btnBox">
            <button class="button" @click="loginRoom">登录</button>
          </div>
          <div class="btnBox">
            <button class="button" @click="goRegister">去注册</button>
          </div>
        </div>
      </div>
    </div>
    <!-- 这里还没改 -->
    <room v-else />
    <!-- <room
      v-else
      :user="user"
      :userList="userList"
      ref="chatroom"
      @sendServer="sendServer"
      :message="message"
      @handleFile="handleFile"
    /> -->
  </div>
</template>

<script>
import Room from './Room'
// import io from 'socket.io-client' //引入socket.io-client
export default {
  name: 'Login',
  components: { Room },
  data() {
    return {
      socket: null,
      isShow: true,
      showWarn: false,
      user: {}, //當前用戶
      userList: [], //好友列表
      message: {},
    }
  },
  methods: {
    returnLogin(){
      this.isShow = true;
    },
    goRegister(){
      this.$router.push('/Register');
    },
    loginRoom() {
      // 1.获取用户名
      const uname = this.$refs.inputUsername.value
      const pword = this.$refs.inputPassword.value
      if (!uname.trim()) {
        alert('请输入用户名')
        return
      }
      if (!pword) {
        alert('请输入密码')
        return
      }
      this.$api.userApi.login(uname,pword).then((result)=>{
        if(result.result){
          this.$api.userApi.getUserById({id:result.id}).then((res)=>{
            this.user = res.user;
          })
          this.$api.friendApi.getList({id:result.id}).then((res)=>{
            this.userList = res.friends;
          })
          this.showWarn = false;
          this.isShow = false;
        }
        else{
          this.showWarn = true;
        }
      })
    },
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
.Login {
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
    align-items: center;
    background-color: #fff;
    .mesBox{
      margin:10px 10px 0px 10px;
      width: 240px;
      height: 80px;
      text-align: left;
    }
    label {
      color: #000;
      font-size: 14px;
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
      .chooseAvatar {
        margin-top: 15px;
      }
      .avatarWrap {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
        border: 1px solid #ccc;
        li {
          cursor: pointer;
          width: 50px;
          height: 50px;
          padding: 7px;
          img {
            width: 50px;
            height: 50px;
          }
          .active {
            border: 3px solid #2980b9;
          }
        }
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
