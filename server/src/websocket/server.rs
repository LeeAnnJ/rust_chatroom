//! `ChatServer` is an actor. It maintains list of connection client session.
//! And manages available rooms. Peers send messages to other peers in same
//! room through `ChatServer`.

use std::{
    collections::{HashMap,},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use actix::prelude::*;
use serde::{Serialize, Deserialize};
use super::{ GrpMessage };
use crate::entity::SendMessage;

extern crate sqlx;
use sqlx::{
    mysql::{MySqlPoolOptions, MySql},
    Pool,FromRow, Row
};

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for chat server communications

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub id: i32,
    pub name: String,
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: i32,
}

/// Send message 
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: i32,
    pub content: String
}

/// `ChatServer` manages chat rooms and responsible for coordinating chat session.
///
/// Implementation is very naïve.
#[derive(Debug)]
pub struct ChatServer {
    sessions: HashMap<i32, Recipient<Message>>,
    clients: HashMap<i32, String>,
    visitor_count: Arc<AtomicUsize>,
    pool: Pool<MySql>
}

impl ChatServer {
    pub fn new(visitor_count: Arc<AtomicUsize>, pool: Pool<MySql>) -> ChatServer {
        // default room
        ChatServer {
            sessions: HashMap::new(),
            clients: HashMap::new(),
            visitor_count,
            pool
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

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_message( &format!("Total visitors {count}"), 0);

        // send id back
        msg.id as usize
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            self.clients.remove(&msg.id);
        }
        
        // send message to other users
        let sName = self.clients.get(&msg.id).unwrap();
        let message = GrpMessage {
            isPublic: true,
            sID: msg.id,
            sName: String::from("Public"),
            rID:0,
            mes: format!("{} leave the room.",sName)
        };
        self.send_message(message.to_string().as_str(), 0);
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(msg.content.as_str(), msg.id);
    }
}

// 私聊信息
#[derive(Debug, Serialize, Deserialize)]
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
        if self.clients.get(&msg.rID).is_some() {
            let id = msg.rID;
            self.send_message_spc(msg.to_string().as_str(), id);
        }
    }
}