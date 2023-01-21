use actix_web::{
    get, post,web, HttpResponse,
    web::Data,
    http::{
        header::ContentType,
    },
};
use serde_json::json;

use crate::appState::AppState;
use crate::entity::{ SendMessage, Relation, IDList };

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

// 获取消息记录 已测试
#[get("/message/getList")]
pub async fn get_message_list (form: web::Query<Relation>, data: Data<AppState>) -> HttpResponse {
    let relation = form.into_inner();
    println!("get message list:{:?}",relation);
    let pool = &data.pool;
    let result = relation.get_record(pool,false).await.unwrap();
    let res = json!({
            "status": 0,
            "data":{"messagelist": result},
            "msg":"sucess"
    });
    HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(res)
}

// 设置已读信息 已测试
#[post("/message/setRead")]
pub async fn set_message_read (form: web::Json<IDList>, data: Data<AppState>) -> HttpResponse {
    let meslist = form.into_inner();
    println!("set message read:{:?}",meslist);
    let pool = &data.pool;
    let result = meslist.set_read(pool, true).await.unwrap();
    let res = if result>0 {
        json!({
            "status": 0,
            "data":{"affect-row": result},
            "msg":"sucess"
        })
    } else {
        json!({
            "status": 400,
            "data":{"result":false},
            "msg": "no record is affected."
        })
    };
    HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(res)
}