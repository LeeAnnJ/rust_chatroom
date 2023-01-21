/**
 * websocket服务器，处理由session发来的不同类型的信息
 */ 
use std::{
    collections::{HashMap,},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    }, 
    i32
};
use actix::prelude::*;
use serde::{Serialize, Deserialize};
use super::{ GrpMessage };


/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// `ChatServer` manages chat rooms and responsible for coordinating chat session.
///
/// Implementation is very naïve.
#[derive(Debug)]
pub struct ChatServer {
    sessions: HashMap<i32, Recipient<Message>>,
    clients: HashMap<i32, String>,
    window_map :HashMap<i32,i32>,
    visitor_count: Arc<AtomicUsize>,
}

impl ChatServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
            clients: HashMap::new(),
            window_map: HashMap::new(),
            visitor_count,
        }
    }
}

impl ChatServer {
    // 向客户端返回消息
    fn send_message(&self, message: &str, skip_id: i32) {
        for (id,_) in self.sessions.iter() {
            if *id != skip_id {
                if let Some(addr) = self.sessions.get(id) {
                    addr.do_send(Message(message.to_owned()));
                }
            }
        }
    }

    fn send_message_spc(&self, message: &str, skip_id: i32) {
        for (id,_) in self.sessions.iter() {
            if *id == skip_id {
                if let Some(addr) = self.sessions.get(id) {
                    addr.do_send(Message(message.to_owned()));
                }
            }
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub id: i32,
    pub name: String,
    pub addr: Recipient<Message>,
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let message = GrpMessage {
            isPublic: false,
            sID: msg.id,
            sName: String::from("Public"),
            rID: 0,
            mes: format!("{} joined chatroom.",msg.name)
        };
        
        // 广播用户已经加入聊天室
        self.send_message(message.to_string().as_str(), 0);
        // 存储用户信息
        self.clients.insert(msg.id, msg.name.clone());
        // 存储用户地址
        self.sessions.insert(msg.id, msg.addr);
        // 存储当前定向
        self.window_map.insert(msg.id, 0);

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_message( &format!("Total visitors {count}"), 0);

        // send id back
        msg.id as usize
    }
}

/// 断开会话
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: i32,
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        // 广播信息：用户离开房间
        let sName = self.clients.get(&msg.id).unwrap();
        let message = GrpMessage {
            isPublic: true,
            sID: msg.id,
            sName: String::from("Public"),
            rID:0,
            mes: format!("{} leave the room.",sName)
        };
        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            self.clients.remove(&msg.id);
            self.window_map.remove(&msg.id);
        }
        self.send_message(message.to_string().as_str(), 0);
    }
}

/// Send message 
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: i32,
    pub content: String
}

/// Handler for Message message.
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(msg.content.as_str(), msg.id);
    }
}

// 私聊信息
#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(Message)]
#[rtype(result = "()")]
pub struct PvtMessage {
    pub isPublic: bool,
    pub sID: i32,
    pub rID: i32,
    pub mes: String
}

impl PvtMessage {
    pub fn to_string(self) -> String {
        let str = String::from("{\"isPublic\": false,")
            +"\"sID\": "+self.sID.to_string().as_str()+","
            +"\"rID\": "+self.rID.to_string().as_str()+","
            +"\"mes\": \""+self.mes.as_str()+"\"}";
        str.clone()
    }
}

impl Handler<PvtMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: PvtMessage, _: &mut Context<Self>) {
        let mut flag = false;
        let message = msg.clone();
        if self.clients.get(&msg.rID).is_some() {
            let map = self.window_map.get(&msg.rID).unwrap();
            if msg.sID == *map {flag=true;}
        }
        if flag {
            let id = msg.rID;
            self.send_message_spc(msg.to_string().as_str(), id);
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join{
    pub user: i32,
    pub map: i32
}

impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        *self.window_map.entry(msg.user).or_insert(0) = msg.map;
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Check{
    pub user: i32,
    pub map: i32
}

impl Handler<Check> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Check, _: &mut Context<Self>) {
        let mut flag = false;
        if self.clients.get(&msg.map).is_some() {
            let map = self.window_map.get(&msg.map).unwrap();
            if msg.user == *map {flag=true;}
        }
        let message = format!("{{\"result\":{}}}",flag);
        self.send_message_spc(message.as_str(),msg.user);
    }
}