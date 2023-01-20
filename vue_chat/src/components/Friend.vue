<template>
    <div class="addF" ref="friend">
        <div class="title">添加好友</div>
        <span class="icon iconfont icon-guanbi" @click="closeTab()"></span>
        <p>按用户名或ID查找好友</p>
        <input type="text" class="user" id="usermes" ref="inputUsermes" />
        <div class="btnBox">
            <button class="button" @click="search">查找</button>
        </div>
        <p>用户是否存在: {{esixt}}</p>
        <div>你要添加的用户为</div>
        <div class="left" v-if="user.ID">: "{{user.uName}}"</div>
        <div class="left" v-else>:无</div>
        <div class="btnBox">
            <button class="button" @click="add">确定添加</button>
        </div>
        <div class="mesBox">
          {{ message }}
        </div>
        <div class="list" v-if="showList">
            <div class="ti">查询列表</div>
            <div class="record" v-for="item in searchList" :key="item.ID">
                <div class="name" v-if="uid!=item.ID" @click="choose(item)">{{item.ID}}:  {{item.uName}}</div>
            </div>
        </div>
    </div>
</template>

<script>
  export default {
    name: 'Friend',
    props: {
        uid:Number  //本机用户id
    },
    data(){
        return{
            esixt: "否",
            message: "",
            friendID: 0,
            showList:false,
            // uid:1,
            // searchList:[{ID:1,uName:"momo"},{ID:2,uName:"nono"}],
            searchList:[],
            user:{} //选择添加的用户
        }
    },
    methods:{
        choose(item){
            this.message = "";
            this.user = item;
        },
        search(){
            const umes = this.$refs.inputUsermes.value;
            // 模糊搜索
            this.$api.userApi.search({name:umes}).then((res)=>{
                
                if(res.user.length == 0){
                    this.esixt = "否";
                }
                else{
                    this.esixt = "是";
                    this.showList = true;
                    this.searchList = res.user;
                }

            })
        },
        add(){
            if(this.esixt == "否"){
                this.message = "用户不存在, 无法添加!";
                return;
            }
            if(!this.user.uName){
                this.message = "未选择用户, 无法添加！";
                return;
            }
            let params = {
                "user": this.uid,
                "friend": this.user.ID,
            };
            this.$api.friendApi.create(params).then((res)=>{
                // console.log(res.result);
                if(res.result){
                    this.message = "["+this.user.uName+"]添加成功！"; 
                    this.$parent.addUser(this.user);    
                }
                else{
                    this.message = "已为好友, 无法添加";
                }
            })
            // this.message = "添加成功！";
            // this.message = "已为好友！";
            // 若添加成功 要刷新好友列表
        },
        closeTab(){
            this.$parent.closeSearch();
        }
    }
  }
</script>

<style lang="scss" scoped>
.addF{
    position: absolute;
    width: 200px;
    height: 340px;
    left: 320px;
    top: 140px;
    background-color: rgb(201, 196, 255);
    border: 3px solid rgb(82, 24, 126);
    border-radius: 5px;
    

    .list {
        position: absolute;
        width: 200px;
        height: 340px;
        top: -2px;
        left:200px;
        background-color: #fff;
        border: 3px solid rgb(82, 24, 126);
        border-radius: 5px;
        .ti{
            height: 20px;
            border-bottom: 2px solid #705a76;
        }
        .name {
            padding-left: 30px;
            text-align: left;
            height: 20px;
            border-bottom: 2px solid rgb(82, 24, 126);
            cursor: pointer;
        }
    }

    .title {
        height: 25px;
        background-color: #d4e7fd;
        border: 3px solid rgb(201, 196, 255);
        border-radius: 10px;
    }
    .btnBox {
      margin-top: 20px;
      .button {
        width: 80px;
        line-height: 20px;
        background-color: #705a76;
        color: #fff;
        border-radius: 10px;
      }
    }
    .mesBox {
        margin-top: 10px;
        text-align: center;
        color: rgb(124, 48, 48);
    }
    .iconfont {
      display: block;
      font-family: iconfont !important;
      font-style: normal;
    }
    .icon {
        position: absolute;
        left: 170px;
        top: 4px;
        font-size: 20px;
        color: red;
    }

    .left {
        height: 20px;
        text-align: left;
        padding-left: 30px;
        margin-top: 5px;
    }
}
</style>