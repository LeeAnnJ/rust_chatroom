<template>
  <div class="Room" ref="room">
    <div class="room-left">
      <div class="user">
        <span class="icon iconfont icon-wode"></span>
        <span class="iname">个人</span>
      </div>
      <div class="friend" @click="showFriend()">
        <span class="icon iconfont icon-fenzu"></span>
        <span class="iname">好友</span>
      </div>
      <div class="group" @click="showGroup()">
        <span class="icon iconfont icon-duihua"></span>
        <span class="iname">群聊</span>
      </div>
      <div class="exit" @click="goLogin()">
        <span class="icon iconfont icon-guanbi1"></span>
        <span class="iname">退出</span>
      </div>
      
    </div>

    <div class="room-center">
      <div class="center-h">
        <p v-if="isGroup">群聊列表</p>
        <p v-else>好友列表</p>
        <p />
      </div>
      <div class="center-b">
        <div v-if="isGroup">
          <li class="user-item" v-for="item in groupList" :key="item.gName" @click="goPublic()">
            <span>{{item.gName}}</span>
          </li>
        </div>
        <div v-else>
          <li class="user-item" v-for="item in userList" :key="item.uName" @click="changeWindow(item.uName)">
            <span>{{item.uName}}</span>
          </li>
        </div>
      </div>
    </div>

    <div class="room-right">
        <div class="ChatContent" ref="chat">
          <div class="title">{{title}}</div>
          <div class="join">
            <!-- 消息框 -->
            <li v-for="(item,index) in messageContent" 
              :key="index"
              :class="{'myMes':item.type===1?true:false, 'otherMes':item.type===2?true:false}">
              <!-- 我方信息 -->
              <div v-if="item.type === 1">
                <span>{{item.msg}}</span> 
                <p class="name">{{item.username}}</p>
                <p class="time">{{item.time}}</p>
              </div>
              <!-- 他人信息 -->
              <div v-if="item.type ===2">
                <span class="content">{{item.msg}}</span>
                <p class="username">{{item.username}}</p>  
                <p class="time">{{item.time}}</p>
              </div>
          </li> 
        </div>
        </div>
        <div class="MessageBox">
            <div class="iconbar"></div>
            <textarea cols="80" rows="5" ref="textarea" @keyup.enter="handlePress"></textarea>
            <button class="sendMessage" @click="sendContentToServe">发送</button>
        </div>
    </div>
    
  </div>
</template>

<script>
  export default {
    name: 'Room',
    props: {
      // user: Object,
      // userList: Array,
      // message: Object,
    },
    computer: {
      userListLength() {
        return this.userList.length
      },
    },
    data() {
      return {
        username: 'test',
        title: '公共聊天室',  //用於聊天框頂部顯示
        isGroup: false, // 用于切换好友群聊列表
        ispublic: true, //用於判斷是否和好友私聊
        messageContent: [],
        content: '',
        user: {username:'test'},
        userList: [{
                "ID": 3,
                "uName": "xixi"
            },{
                "ID": 2,
                "uName": "nono"
            }
        ],
        groupList:[{"gName":"公共聊天室"}]
      }
    },
    methods: {
      showGroup(){
        this.isGroup = true;
      },
      showFriend(){
        this.isGroup = false;
      },
      goLogin(){
        this.$parent.returnLogin();
      },
      changeWindow(uName){
        this.ispublic = false;
        this.title = uName;
        // 清屏
        this.messageContent = [];
        // 更改title 更改聯繫人
      },
      goPublic(){
        this.title = "公共聊天室";
        this.ispublic = true;
        // 清屏
        this.messageContent = [];
      },
      handlePress(){
        let tmp = this.$refs.textarea.value
        //需要一个判断是否全为换行符的
        // if(tmp == "\n" || tmp == "\n\n" || tmp == "\n\n\n"){
        //   console.log("Empty");
        //   return
        // }
        this.sendContentToServe()
      },
      sendContentToServe() {
        // 获取到聊天的内容
        this.content = (this.$refs.textarea.value).trim()
        this.$refs.textarea.value = ''
        if (!this.content) {
          return alert('请输入内容')
        }
        
        let newValue = {username:this.username, msg:this.content, time:this.getTime()};
        this.handleMessageBox(newValue);
        // 发送给服务器
        // sendServer是Login里的那个父函数
        // this.$emit('sendServer', this.content)
      },
      handleMessageBox(newValue) {
        // console.log(newValue);
        if (newValue.username === this.user.username) {
          //是自己发的信息
          this.messageContent.push({ ...newValue, type: 1 })
          // this.messageContent.push({ ...newValue, type: 2 }) //debug用
        } else {
          //是别人发的信息
          this.messageContent.push({ ...newValue, type: 2 })
        }
        // console.log(this.messageContent);
      },
      handleScrollBottom() {
        const scoll = this.$refs.chat
        scoll.scrollTop = scoll.scrollHeight-355;
      },
      checkTime(obj){
        if(obj<10){
          obj = "0"+obj;
        }
        return obj;
      },
      getTime(){
        var now = new Date();
        var h = now.getHours(),
            m = now.getMinutes(),
            s = now.getSeconds(),
            h = this.checkTime(h),
            m = this.checkTime(m),
            s = this.checkTime(s);
        return h+":"+m+":"+s;
      }
    },
    updated() {
      // 聊天定位到底部
      this.handleScrollBottom()
    },
  }
</script>

<style lang="scss" scoped>
.Room{
  width: 900px;
  height: 500px;
  margin: 50px auto;
  display: flex;
  .room-left {
    background-color: #474574;
    width: 50px;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 10px;
    .iconfont {
      display: block;
      font-family: iconfont !important;
      font-style: normal;
    }
    .user,.friend,.group,.exit{
      padding-bottom: 10px;
      cursor: pointer;
    }
    .exit{
      position: relative;
      top:240px;
    }
    .iname{
      height: 18px;
      color: white;
    }
    .icon{
      color: white;
      height: 30px;
      font-size: 24px;
      
    }
    .active {
      color: #ecf0f1;
    }
    img {
      width: 40px;
    }
  }
  .room-center {
    width: 200px;
    background-color: #e6e5e5;
    color: #000;
    display: flex;
    flex-direction: column;
    .center-h {
      padding: 9.6px 10px;
      height: 20px;
      border-bottom: 1px solid #e5e5e58c;
      box-shadow: 1px 1px 1px #b2c0c9;
      display: flex;
      align-items: center;
      img {
        width: 50px;
        height: 50px;
      }
    }
    .center-b {
      flex: 1;
      .user-item {
        padding: 5px 10px;
        height: 20px;
        display: flex;
        align-items: center;
        margin-bottom: 1px;
        border-bottom: solid 1px #b2c0c9;
        span {
          margin-left: 5px;
        }
      }
    }
  }

  .room-right {
    flex: 1;
    background-color: #f6f6f6;
    display: flex;
    flex-direction: column;
    .ChatContent {
      height: 355px;
      overflow: auto;
      .title{
        margin-top: 3px;
        height: 26px;
        border-bottom: solid 1px #b2c0c9;
      }
      .join {
        height: 325px;
        // margin-top: 30px;
      li {
          padding-left: 10px;
          padding-bottom: 15px;
        }
      .myMes {
        display: flex;
        justify-content: flex-end;
        margin-top: 15px;
        margin-bottom: 15px;
        div {
          display: flex;
          position: relative;
          span {
            box-sizing: border-box;
            display: inline-block;
            border-radius: 5px;
            line-height: 20px;
            background-color: #9eea6a;
            color: #000;
            padding: 5px;
            margin-right: 10px;
            min-width: 40px;
          }
          .name {
            position: absolute;
            right: 10px;
            top: -28px;
            font-size: 13px;
            color: #7e7e7e;

          }
          .time {
            position: absolute;
            right: 10px;
            bottom: -28px;
            font-size: 13px;
            color: #b2b2b2;
          }
        }
      }
      .otherMes {
        position: relative;
        display: flex;
        justify-content: flex-start;
        margin-top: 15px;
        margin-bottom: 15px;
        div {
          display: flex;
          position: relative;

          .username {
            position: absolute;
            left: 1px;
            top: -28px;
            font-size: 13px;
            color: #7e7e7e;
          }

          .content {
            // margin-top: 12px;
            box-sizing: border-box;
            display: inline-block;
            border-radius: 5px;
            line-height: 20px;
            background-color: #fff;
            color: #000;
            padding: 5px;
            // margin-left: 10px;
            border: 1px solid #ccc;
            min-width: 40px;
          }

          .time {
            position: absolute;
            left: 1px;
            bottom: -28px;
            font-size: 13px;
            color: #b2b2b2;
          }
        }
      }
      }
    }
    .MessageBox {
      position: relative;
      flex: 1;
      background-color: #fff;
      .iconbar {
        height: 25px;
        padding-top: 6px;
        display: flex;
        background-color: #d8d4d4;
      }
      textarea {
        border: none;
        resize: none;
        outline: none;
        font-size: 15px;
        padding-left: 9px;
      }
      .sendMessage {
        position: absolute;
        bottom: 10px;
        right: 10px;
        padding: 4px 10px;
        background-color: #f5f5f5;
        border: 1px solid #ccc;
      }
    }
  }
}
</style>