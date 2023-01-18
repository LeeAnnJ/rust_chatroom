use std::{convert::Infallible, io};
use actix::{Actor,StreamHandler};
use actix_files::{Files, NamedFile};
use actix_web::{
    get, post, App, HttpRequest, HttpServer, web, error, HttpResponse, middleware, Either, Responder, Result,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
};
use actix_web_actors::ws::{self, Message, WebsocketContext};
use serde::Deserialize;
use async_stream::stream;

pub struct WsConn {
    pub nick: String,
}

impl Actor for WsConn {
    type Context = WebsocketContext<Self>;
    /// 连接上
    fn started(&mut self, _: &mut Self::Context) {
        println!("{} join!", self.nick);
    }

    /// 断开连接
    fn stopped(&mut self, _: &mut Self::Context) {
        println!("{} exit!", self.nick);
    }
}

impl StreamHandler<Result<Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, item: Result<Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(Message::Text(text)) => ctx.text(text),
            Ok(Message::Ping(msg)) => ctx.pong(&msg),
            Ok(Message::Binary(bin)) => ctx.binary(bin),
            Ok(Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}

#[get("/ws/{nick}")]
async fn index(req: HttpRequest, stream: web::Payload, params: web::Path<String>) 
-> HttpResponse {
    let conn = WsConn {
        nick: params.to_string(),
    };
    let resp = actix_web_actors::ws::start(conn, &req, stream);
    match resp {
        Ok(ret) => ret,
        Err(e) => e.error_response(),
    }
}

// 在这里指明路径参数，并使用元组接收，同时确定类型
async fn getMyfuction(path:web::Path<String,>) -> HttpResponse {
    println!("get hello path");
    HttpResponse::Ok()
    .content_type(ContentType::plaintext())
    .body(format!("Hello {}!", path.into_inner()))
}


// #[get("/t1/{id}/{name}")]
// async fn get1(path: web::Path<(i32, String)>) -> String {
//     println!("{:?}", path.into_inner());
//     "ok".to_string()
// }

#[derive(Debug, Deserialize)]
struct User {
    id: i32,
    name: String,
}

// 使用结构体接收
#[get("/t2/{name}/{name}")]
async fn get2(user: web::Path<User>) -> String {
    println!("{:?}", user.into_inner());
    "ok".to_string()
}

// 使用查询参数解析，不建议，因为这是类型不安全的
// #[get("/t3/{id}/{name}")]
// async fn get3(req: HttpRequest) -> String {
//     let id: i32 = req.match_info().query("id").parse().unwrap();
//     let name: String = req.match_info().query("name").parse().unwrap();
//     println!("{}, {}", id, name);
//     "ok".to_string()
// }


// 查询参数绑定
// #[get("/t4")]
// async fn get4(user: web::Query<User>) -> String {
//     println!("{:?}", user.into_inner());
//     "ok".to_string()
// }

// 使用json解析
// #[post("/t1")]
// async fn post1(user: web::Json<User>) -> HttpResponse {
//     println!("get post t1");
//     HttpResponse::Ok()
//     .content_type(ContentType::plaintext())
//     .body(format!("get post path!: {:?}", user.into_inner()))
// }

// 使用json解析
// #[post("/t2")]
// async fn post2(user: web::Form<User>) -> String {
//     println!("{:?}", user.into_inner());
//     "ok".to_string()
// }

/// 请求连接：
/// get@/t1/123/aaa
/// get@/t2/123/aaa
/// get@/t3/123/aaa
/// get@/t4?id=123&name=aaa
/// post@/t1 {"id": 123,"name": "aaa"}
/// post@/t2 id: 123, name: "aaa"
#[actix_web::main]
async fn start_actix() -> std::io::Result<()> {
    // 创建一个json解析配置，并用于处理json解析   

    let json_config = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                .into()
        });

    println!("starting HTTP server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(get1)
            .service(get2)
            .service(get3)
            .service(get4)
            .service(web::resource("/hello/{name}")
                .route(web::get().to(getMyfuction)))
            // 这个配置只会影响json解析，不会影响其他类型的解析
            .app_data(json_config.clone())
            .service(post1)
            .service(post2)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn main(){
    start_actix();
}