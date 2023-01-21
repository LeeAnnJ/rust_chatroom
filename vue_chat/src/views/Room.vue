<template>
  <div class="Room" ref="room">
    <friend v-if="addF" :uid="user.ID" />
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
        <div class="bar" v-if="isGroup">群聊列表</div>
        <div v-else>
          <div class="bar">好友列表</div>
          <span class="addf iconfont icon-faqi" @click="addFriends"></span>
        </div>
        
        
      </div>
      <div class="center-b">
        <div v-if="isGroup">
          <li class="user-item" v-for="item in groupList" :key="item.gName" @click="goPublic()">
            <span>{{item.gName}}</span>
          </li>
        </div>
        <div v-else>
          <li class="user-item" v-for="item in userList" :key="item.uName" @click="changeWindow(item.ID,item.uName)">
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
              <div v-if="item.type === 2">
                <span class="mes">{{item.msg}}</span>
                <p class="username">{{item.username}}</p>  
                <p class="time">{{item.time}}</p>
              </div>
          </li> 
        </div>
        </div>
        <div class="MessageBox">
            <div class="iconbar">
              <span class="history iconfont icon-xiaoxi" v-if="!ispublic" @click="showHistory()"/>
              <history v-if="showHis" :uid="user.ID" :uName="user.uName" :fid="wID" :fName="title"/>
            </div>
            <textarea cols="80" rows="5" ref="textarea" @keyup.enter="handlePress"></textarea>
            <button class="sendMessage" @click="sendContentToServe">发送</button>
        </div>
    </div>
    
  </div>
</template>

<script>
import Friend from '@/components/Friend.vue';
import History from '@/components/History.vue';
  export default {
    name: 'Room',
    components:{Friend,History},
    props: {
      user: Object,
      userList: Array,
      // message: Object,
    },
    computer: {
      userListLength() {
        return this.userList.length
      },
    },
    data() {
      return {
        username: '', // 当前用户名
        title: '公共聊天室',  //用於聊天框頂部顯示
        isGroup: false, // 用于切换好友群聊列表
        ispublic: true, //用於判斷是否和好友私聊
        wID: 0,  // 当私聊时好友的ID
        wName: '',  //私聊好友名称
        addF: false, // 控制好友添加窗口
        showHis: false,
        messageContent: [], //当前视窗的消息记录
        content: '',  
        groupList:[{"gName":"公共聊天室"}],
        reswslist:[],  //法二 缓存还未发送的ws消息
        ressqllist:[]   //法二 缓存还未发送的ws消息对应的数据库信息
      }
    },
    mounted(){
      this.username = this.user.uName;
    },
    destroyed(){
      this.$parent.close();
    },
    methods: {
      showHistory(){
        this.showHis = !this.showHis;
      },
      showGroup(){
        this.isGroup = true;
      },
      showFriend(){
        this.isGroup = false;
      },
      goLogin(){
        this.$parent.returnLogin();
      },
      changeWindow(ID,uName){
        this.ispublic = false;
        // 更改title 更改聯繫人
        this.title = uName;
        this.wID = ID;
        this.wName = uName;
        // 清屏
        this.messageContent = [];
        let params = {
          "user":this.user.ID,
          "friend":ID
        };
        // ws传输映射 /turn ID
        this.$parent.send("/turn "+ID);
        let ids = [];
        // 获取未读信息
        this.$api.mesApi.getList(params).then((res)=>{
          // console.log(res.messagelist);
          let allmes = res.messagelist;
          for(var i=allmes.length-1; i>=0; i-- ){
            // console.log(allmes[i]);
            if(!allmes[i].isread && allmes[i].sID==ID){
              let newValue={
                "username":uName, 
                "msg": allmes[i].mes, 
                "time": this.parseTime(allmes[i].sTime),
              };
              this.handleMessageBox(newValue);
              ids.push(allmes[i].mesID);
            }
          }
          if(ids.length>0){
            //标记为已读
            this.$api.mesApi.setRead({"ids":ids}).then((res)=>{
              // console.log(res);
            });
          }
        })

      },
      goPublic(){
        this.title = "公共聊天室";
        this.ispublic = true;
        // 清屏
        this.messageContent = [];
        //ws 传输映射 /turn 0
        this.$parent.send("/turn 0");
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
      // 数据库加入消息时 如果有单引号要处理
      format(content){
        return content.replace(/'/g,"''");
      },
      sendContentToServe() {
        // 获取到聊天的内容
        this.content = (this.$refs.textarea.value).trim()
        this.$refs.textarea.value = ''
        if (!this.content) {
          return alert('请输入内容')
        }
        
        let newValue = {"username":this.username, "msg":this.content, "time":this.getTime()};
        this.handleMessageBox(newValue);
        // ws通信 按类型分发
        // 群聊消息
        if(this.isGroup){
          // ws传输
          let params={
            "ispublic":true,
            "sID":this.user.ID,
            "sName":this.user.uName,
            "rID":0,
            "mes":this.content,
            "time":this.getTime(),
          };
          this.$parent.send(JSON.stringify(params));
        }
        else{
          // 法二部分：ws询问点对点
          this.$parent.send("/check "+this.wID);
          // ws传输信息
          let params={
            "ispublic":false,
            "sID":this.user.ID,
            "rID":this.wID,
            "mes":this.content,
            "time":this.getTime(),
          }
          // 法二部分 下面这条要删掉
          // this.$parent.send(params);
          // 法二部分 要缓存信息加入下面这段
          this.reswslist.push(params);
          let sqlp = {
            "send":this.user.ID,
            "recieve":this.wID,
            "mes":this.format(this.content),
            "isRead":false
          };
          this.ressqllist.push(sqlp);
        }

      },
      handleMessageBox(newValue) {
        // console.log(newValue);
        if (newValue.username == this.user.uName) {
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
      },
      closeSearch(){
        this.addF = false;
        console.log("close");
      },
      addFriends(){
        this.addF = !this.addF;
        console.log("add");
      },
      addUser(user){
        this.userList.push(user);
        console.log("here");
        console.log(this.userList);
      },
      parseTime(time){
        if (time==null) return "暂无";
        var date = new Date(time);
        let monthString = date.getMonth()+1>=10? String(date.getMonth()+1): '0'+(date.getMonth()+1);
        let dtString = date.getDate()>=10? String(date.getDate()):'0' + date.getDate(); 
        let hourString = date.getHours()>=10? String(date.getHours()):'0'+date.getHours();
        let minuString = date.getMinutes()>=10? String(date.getMinutes()):'0'+date.getMinutes();
        let secondString = date.getSeconds()>=10? String(date.getSeconds()):'0'+date.getSeconds();
        let str=date.getFullYear()+'-'+monthString+'-'+dtString+' '+hourString+':'+minuString+':'+secondString;
        return str;
      },
      handlewsmes(obj){
        // 法二部分 判断是不是check返回的信息
        // 如果是 流程结束后return
        if("result" in obj){
          let result = obj.result;
          // 要不要判断check返回的对应id？
          // 取check的结果 x
          let resws = reswslist.pop();
          let ressql = ressqllist.pop();
          if(result){
            this.$parent.send(JSON.stringify(resws));
            ressql.isRead = true;
          }
          this.$spi.mesApi.addmessage(ressql).then((res)=>{
              console.log(res);
          });
          return;
        }
        // 正常部分
        if(obj.isPublic == this.ispublic){
          if(obj.isPublic){
            // 接收群聊信息
            let newValue = {"username":obj.sName, "msg":obj.mes, "time":obj.time};
            this.handleMessageBox(newValue);
          }
          else{
            if(obj.sID != this.wID) return;
            // 接收私聊信息
            let newValue = {"username":this.wName,"msg":obj.mes, "time":obj.time};
            this.handleMessageBox(newValue);
          }
        }
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
      // cursor: pointer;
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
      .history{
        cursor: pointer;
      }
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
      // display: flex;
      align-items: center;
      .addf{
        position: relative;
        left:70px;
        bottom: 18px;
        cursor: pointer;
      }
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
        cursor: pointer;
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
        padding-top: 15px;
        padding-bottom: 15px;
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
        padding-top: 15px;
        padding-bottom: 15px;
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

          .mes {
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
            width: 120px;
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
        padding-top: 3px;
        display: flex;
        background-color: #d8d4d4;
        .history {
          margin-left: 10px;
          font-size: 23px;
        }
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