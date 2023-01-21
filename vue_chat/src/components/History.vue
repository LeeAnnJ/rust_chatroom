<template>
    <div class="history" ref="history">
        <div class="title">与{{fName}}的聊天记录</div>
        <div class="contain">
            <div class="container">
                <div class="single" v-for="item in mes" :key="item.mesID">
                    <div class="user">{{item.sName}} {{item.sTime}}</div>
                    <div class="content">{{item.mes}}</div>
                </div>
            </div>
        </div>
        <div class="exit iconfont icon-guanbi" @click="closeTab()"  />
    </div>
</template>

<script>
  export default {
    name: 'History',
    props: {
        uid:Number,
        uName:String,
        fid:Number,
        fName:String
    },
    data(){
        return{
            mes:[]
        }
    },
    mounted(){
        let params = {
            "user":this.uid,
            "friend":this.fid,
        };
        this.$api.mesApi.getList(params).then((res)=>{
            // mes = res.messagelist;
            console.log(res.messagelist.length);
            // let cnt = 0;
            for(var i=res.messagelist.length-1; i>=0 ;i--){
                // if(cnt==20) break;
                // cnt++;
                let sName = res.messagelist[i].sID==this.uid?this.uName:this.fName;
                let sTime = this.$parent.parseTime(res.messagelist[i].sTime);
                let text =  res.messagelist[i].mes;
                let single = {
                    "sName": sName,
                    "sTime": sTime,
                    "mes": text
                };
                this.mes.push(single);
            }
        });
    },
    methods:{
        closeTab(){
            this.$parent.showHistory();
        }
    }
  }
</script>

<style lang="scss" scoped>
.history {
    position: absolute;
    left: 30px;
    bottom: 60px;
    width: 400px;
    height: 400px;
    border: 2px solid black;
    border-radius: 5px;
    background-color: rgb(236, 236, 236);
    .title {
        margin-left: 10px;
        margin-right: 10px;
        border-bottom: 4px solid white;
    }
    .exit {
        font-size: 20px;
        position: relative;
        top: 13px;
        cursor: pointer;
    }
    .contain {
        position: relative;
        width: 380px;
        height: 320px;
        left: 10px;
        top: 5px;
        overflow: auto;
        background-color: white;
        .container {
        
        
        width: 360px;
        height: 320px;
        background-color: white;
        .single{
            // background-color: goldenrod;
            text-align: left;
            font-size: 18px;
            padding-left: 10px;
            margin-bottom: 2px;
            border: 1px solid grey;
            border-radius: 3px;
            .content {
                padding-left: 10px;
            }
        }
    }
    }
    
}
</style>