<template>
  <div class="Room" ref="room">
    <div class="room-left">
    </div>

    <div class="room-center">
      <div class="center-h">
        <p>好友列表</p>
        <p />
      </div>
      <div class="center-b">
        <!-- <ul>
          <li class="user-item" v-for="item in userList" :key="item.username">
            <img :src="require('@/assets/avatar/' + item.avatar)" alt />
            <span>{{item.username}}</span>
          </li>
        </ul> -->
      </div>
    </div>

    <div class="room-right">
        <div class="ChatContent" ref="chat">
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
        messageContent: [],
        content: '',
        user: {username:'test'}
      }
    },
    methods: {
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
    .icon-liaotianqingqiu,
    .icon-yonghu {
      font-size: 24px;
      padding-top: 10px;
      cursor: pointer;
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
      .user-item-name {
        padding: 5px 10px;
        height: 40px;
        display: flex;
        align-items: center;
        border-bottom: 1px solid #e5e5e58c;
        box-shadow: 1px 1px 1px #2980b9;
        font-size: 17px;
      }
      .user-item {
        padding: 5px 10px;
        height: 40px;
        display: flex;
        align-items: center;
        img {
          width: 40px;
          height: 40px;
        }
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
      .join {
        height: 325px;
        margin-top: 30px;
      li {
          padding-left: 10px;
          padding-bottom: 5px;
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