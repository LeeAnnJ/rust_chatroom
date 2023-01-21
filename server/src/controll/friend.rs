use actix_web::{
    get, post, web, HttpResponse,
    web::Data,
    http::{
        header::{ ContentType},
    },
};
use serde_json::json;

use crate::appState::AppState;
use crate::entity::{Relation,ReqID};

// 添加好友关系 已测试
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

// 获取好友列表 已测试
#[get("/friend/getList")]
pub async fn get_list (form: web::Query<ReqID>, data: Data<AppState>) -> HttpResponse {
    let id = form.into_inner();
    println!("get firends list of :{:?}",id);
    let pool = &data.pool;
    let result = id.get_friend_list(pool).await.unwrap();
    let res = json!({
            "status": 0,
            "data":{"friends": result},
            "msg":"sucess"
    });
    HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(res)
}