use std::ptr::null;

use serde::{Serialize, Deserialize};
use actix_web::{
    get, post, HttpRequest, HttpServer, web, error, HttpResponse, middleware, Either, Responder, Result, 
    web::Data,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
};
use serde_json::json;
use async_stream::stream;

extern crate sqlx;
use sqlx::{
    mysql::{MySqlPoolOptions, MySql},
    Pool,Error,FromRow, Row
};

use chrono::prelude::*;

use crate::appState::AppState;
use crate::entity::{ SendMessage, Relation };

// 添加消息记录
#[post("/message/addmessage")]
pub async fn add_message (form: web::Json<SendMessage>, data: Data<AppState>) -> HttpResponse {
    let message = form.into_inner();
    println!("add message record:{:?}",message);
    let pool = &data.pool;
    let result = message.add_record(pool).await.unwrap();
    let res = if result>0 {
        json!({
            "status": 0,
            "data":{"result": true},
            "msg":"sucess"
        })
    } else {
        json!({
            "status": 400,
            "data": {"result": false},
            "msg": "fail to add message record!"
        })
    };
    HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(res)
}

// 添加消息记录
#[get("/message/getList")]
pub async fn get_message_list (form: web::Query<Relation>, data: Data<AppState>) -> HttpResponse {
    let relation = form.into_inner();
    println!("get message list:{:?}",relation);
    let pool = &data.pool;
    let result = relation.get_record(pool).await.unwrap();
    let res = json!({
            "status": 0,
            "data":{"messagelist": result},
            "msg":"sucess"
    });
    HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(res)
}