use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};

use actix::*;
use actix_files::{Files, NamedFile};
use actix_web::{
    middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;

pub mod server;
pub use server::PvtMessage;
pub mod session;
pub use session::{GrpMessage};

use crate::entity::SimpleUser;

/// Entry point for our websocket route
pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
    data: web::Path<SimpleUser>
) -> Result<HttpResponse, Error> {
    let user = data.into_inner();
    match ws::start(
        session::WsChatSession {
            id: user.ID,
            hb: Instant::now(),
            name: user.uName,
            room: 0,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    ) {
        Ok(conn) =>{
            log::info!("deal a link request.");
            return Ok(conn)
        },
        Err(e) => {
            log::error!("fail to deal the link reqest at websocket layer!");
            println!("error: {:?}",&e);
            return Err(e)
        }
    };
}

// 获取当前在线人数
pub async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {current_count}")
}