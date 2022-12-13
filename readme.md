# rust大作业选题说明——在线聊天室实现

## 简介

在线聊天室是一个经典的集网络通信与多线程并发的程序设计项目，本课程大作业考虑到两人组的工作量实现的OJ系统不如三人及以上组完善，因此考虑实现在线聊天室作为课程作业.

## 记录板
221213 大概做了消息结构体，接下来需要服务端辨别客户身份发送消息
221206 TCP客户和服务端通信的功能实现了，但是在界面展示的效果不太好

## 预计实现功能

### 基本功能
1. 使用网络通信完成客户端到服务器端、服务器端到客户端的信息交互。
2. 通过数据库存储支持聊天记录储存功能
3. 客户端展示好友列表，并支持通过添加好友发起会话功能。
4. 使用多线程支持多人群聊和多会话并行处理功能。

### 若时间允许完成的更多功能
1. 制作`rust CLI`客户端或简单的`Web`前端
2. 实现离线消息接收功能。