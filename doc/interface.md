# 关于接口定义

返回的数据先考虑正常情况下应该返回的数据，再根据不同情况加入加异常处理。

## 网络层部分：http开头
### 用户信息相关

#### 1.根据用户名查询用户信息
请求地址：`/user/getUserByName` ，请求方式：`get`  
可以用在注册时检查是否有重名，直接**精准查询**就可以

请求参数：
```json
{
    "name": "xxx"
}
```
若数据库有相关账户，返回：
```json
{
    "users":[{
        "ID": "xxx",
        "PW": "xxx",
        "uName": "xxx"
    },{
        "ID": "xxx",
        "PW": "xxx",
        "uName": "xxx",
    },{...}
    ]
}
```
若数据库没有相关信息，返回：
```json
{
    "status": 204,
    "data":{"users":[]},
    "msg": "no such name."
}
```

#### 2.用户注册
请求地址：`/user/registor`，请求方式：`post`  
输入用户名和密码，返回为用户分配的ID  
注册前要注意用户名不能重名

请求参数：
```json
{
    "name": "xxx",
    "password": "xxx"
}
```
成功响应数据：
```json
{
    "id": "xxx",
    "result": true
}
```

#### 3.用户登录
请求地址：`/user/login/{name}/{pass}`，请求方式：`get`  
因为用户不重名，所以其实用账号还是用户名登录都无所谓

请求参数：(写在地址上了)
```json
{
    "name":"xxx",
    "password": "xxx"
}
```
若登录成功，返回：
```json
{
    "result": true,
    "id": 000
}
```
若登录失败，返回：
```json
{
    "result": false
}
```

#### 4.用户修改密码：
请求地址：`/user/alterPassword`，请求方式：`post`  
这个接口不是很重要，留一个 想做就做

请求参数：
```json
{
    "id": 0,
    "newPW": "xxx"
}
```
没有需要返回的数据。
#### 5.搜索用户信息
请求地址：`/user/search`，请求方式：`get`  
这个接口用于添加好友时搜索相关用户信息，通过账号搜索或用户名搜索都可以  
因此需要匹配账号和用户名两个字段，用户名需要**模糊匹配**

请求参数:
```json
{
    "name": "xxx"
}
```
响应数据：
```json
{
    "users":[{
        "ID": "xxx",
        "PW": "xxx",
        "uName": "xxx"
    },{
        "ID": "xxx",
        "PW": "xxx",
        "uName": "xxx",
    },{...}
    ]
}
```

#### 6.根据用户账号查询用户信息：
只是觉得应该有这么一个接口  
请求地址：`/user/getUserById`，请求方式：`get`

请求参数：
```json
{
    "id": 0
}
```
成功响应数据：
```json
{
    "user":{
        "ID": 0,
        "PW": "xxx",
        "uName": "xxx"
    }
}
```

### 好友信息相关
#### 1.添加好友
请求地址：`/friend/create`，请求方式：`post`  
输入两个好友的账号，建立好友信息

请求参数：
```json
{
    "user": "请求接口一方的id",
    "friend": "添加的好友id"
}
```
若成功,返回数据：
```json
{
    "result": true
}
```

#### 2.展示好友列表
请求地址：`/firend/getList`，请求方式：`get`

请求参数：
```json
{
    "id": "xxx"
}
```
成功响应数据：
```json
{
    "friends": [{
        "ID": "xxx",
        "uName": "xxx"
    },{
        "ID": "xxx",
        "uName": "xxx"
    },{...}
    ]
}
```
### 聊天记录相关
#### 1.添加聊天记录
请求地址：`/message/addmessage`，请求方式：`post`  
用户每发送一条消息，向数据库增加一条消息记录

请求参数：
```json
{
    "send": "发送方id",
    "recieve": "接收方id",
    "message": "xxxx",
}
```
不需要返回数据。
#### 2.将消息设置为已读状态
请求地址：`/message/setRead`，请求方式：`post`  
理想的处理方式应该支持一次请求将多个消息设为已读

请求参数：
```json
{
    "ids":["xxx","xxx","xxx"],
}
```
若成功，返回数据
```json
{
    "status":0,
    "data":{
        "affect_row": 1
    },
    "msg": "success"
}
```
若数据库改动的行数为0，返回数据
```json
{
    "status": 400,
    "data":{"result":false},
    "msg": "no record is affected."
}
```

#### 3.拉取信息列表
请求地址：`/message/getList`，请求方式：`get`  
查询两个用户间的聊天记录,按时间降序排序，即第一条为最新消息

请求参数：
```json
{
    "user": "请求一方的id",
    "friend": "对方的id",
}
```
返回数据：
```json
{
    "messageList":[{
        "mesID": 0,
	    "sID": 0,
        "rID": 0,
        "mes": "xxx",
        "sTime": "yyyy-MM-dd hh:mm:ss",
        "isread": true,
    },{
        "mesID": 0,
	    "sID": 0,
        "rID": 0,
        "mes": "xxx",
        "sTime": "xxx",
        "isread": true,
    },{...}]
}
```

## 传输层部分：ws开头
先规定websocket需要传输的数据结构：

客户信息：用于记录连接状态
```json
{
    "ID": 1,
    "uName": "xxx"
}
```
群聊信息：
```json
{
    "isPublic": true,
    "sID": 0,
    "sName": "xxx",
    "rID": 0,
    "mes": "xxx",
    "time": "xxx",
}
```
私聊信息：
```json
{
    "isPublic": false,
    "sID": 0,
    "rID": 0,
    "mes": "xxx",
    "time": "xxx"
}
```

1. 与服务器连接 接口地址：`ws://127.0.0.1:8080/link/{ID}/uName`  
2. 发送消息  
   发送数据格式按上面发送
   ```
   {"isPublic":true,"sID":0,"rTD":0,"mes":"xxx"}
   ```
3. 创建聊天室 接口地址：`/createRoom/{roomName}` (暂时搁置)
4. 获取当前建立的聊天室：`/list` (暂时搁置)
5. 进入聊天室：`/enter/{userName}/{roomName}`（暂时搁置）
6. 删除聊天室：`/delete/{roomName}` （暂时搁置）