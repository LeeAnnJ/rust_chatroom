# 关于接口定义

返回的数据先考虑正常情况下应该返回的数据，之后再加异常处理

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
    "id": "xxx"
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
    "id": "xxx",
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
    "text": "xxx"
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
    "id": "xxxx"
}
```
成功响应数据：
```json
{
    "user":{
        "ID": "xxx",
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
不需要返回数据。

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
    "message":{
	    "send": "发送方id",
        "recieve": "接收方id",
        "message": "xxxx",
        "time": "yyyy-MM-dd hh:mm:ss"
    }
}
```
不需要返回数据。
#### 2.将消息设置为已读状态
请求地址：`/message/setRead`，请求方式：`post`  
理想的处理方式应该支持一次请求将多个消息设为已读

请求参数：
```json
{
    "mesID":["xxx","xxx","xxx"],
}
```
不需要返回数据

#### 3.拉取信息列表
请求地址：`/message/getList`，请求方式：`get`  
查询两个用户间的聊天记录,按时间排序。

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
        "mesID": "xxx",
	    "sID": "xxx",
        "rID": "xxx",
        "mes": "xxx",
        "sTime": "xxx",
    },{
        "mesID": "xxx",
	    "sID": "xxx",
        "rID": "xxx",
        "mes": "xxx",
        "sTime": "xxx",
    },{...}]
}
```

## 传输层部分：ws开头
我目前不是很确定这部分的接口，如果遇到问题的话之后调整

1. 与服务器连接 接口地址：`/link/{userID}`
2. 创建聊天室 接口地址：`/createRoom/{roomName}`
3. 获取当前建立的聊天室：`/list`
4. 进入聊天室：`/enter/{userName}/{roomName}`
5. 删除聊天室：`/delete/{roomName}`