use std::{
    time::{Duration, Instant}
};
use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Serialize, Deserialize};
use serde_json::{ self, json, Value };

use super::server;
use super::server::PvtMessage;

// 心跳检测时间
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
// 超时设定
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

// 群聊信息结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct GrpMessage {
    pub isPublic: bool,
    pub sID: i32,
    pub sName: String,
    pub rID: i32,
    pub mes: String
}

impl GrpMessage {
    pub fn to_string(self) -> String {
        let res = String::from("{\"isPublic\": true,")
            +"\"sID\": "+self.sID.to_string().as_str()+","
            +"\"sName\": \""+self.sName.as_str()+ "\","
            +"\"rID\": 0,"
            +"\"mes\": \""+self.mes.as_str()+"\"}";
        res
    }
}

#[derive(Debug)]
pub struct WsChatSession {
    pub id: i32,     // 用户账号
    pub hb: Instant, // 心跳检测时间
    pub name: String, // 用户名
    pub addr: Addr<server::ChatServer>, // 服务器地址
}

impl WsChatSession {
    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");
                // notify chat server
                act.addr.do_send(server::Disconnect { id: act.id });
                // stop actor
                ctx.stop();
                // don't try to send a ping
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
                id: self.id,
                name: self.name.as_str().to_string()
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res as i32,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<server::Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        log::debug!("WEBSOCKET MESSAGE: {msg:?}");
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                // we check for /sss type of messages
                let value:Value = serde_json::from_str(m).unwrap();
                let ispublic = &value["isPublic"];
                                // send message to chat server
                if *ispublic == json!(true){
                    let msg: GrpMessage = serde_json::from_str(m).unwrap();
                    self.addr.do_send(server::ClientMessage {
                        id: msg.sID,
                        content: msg.to_string()
                    })
                } else {
                    let msg: PvtMessage = serde_json::from_str(m).unwrap();
                    self.addr.do_send(msg)
                }

            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
