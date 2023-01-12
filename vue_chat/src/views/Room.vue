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
        <div class="ChatContent">

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
  name: 'Room'
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
      height: 300px;
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