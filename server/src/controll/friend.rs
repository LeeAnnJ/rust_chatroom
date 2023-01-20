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

use crate::appState::AppState;
use crate::entity::Relation;

// 添加好友关系
#[post("/friend/create")]
pub async fn create_friend (form: web::Json<Relation>, data: Data<AppState>) -> HttpResponse {
    let relation = form.into_inner();
    println!("create relation:{:?}",relation);
    let pool = &data.pool;
    let result = relation.create_relation(pool).await.unwrap();
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
            "msg": "fail to create the relationship,parhaps the relationship has been created."
        })
    };
    HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(res)
}