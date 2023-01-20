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
    mysql::{ MySql },
    Pool,Error,FromRow, Row
};

use crate::user::UserInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqID {
    pub id: i32
}

impl ReqID {
    // 登录的数据库操作
    pub async fn search_user_id (self, pool: &Pool<MySql>) -> Result<UserInfo, sqlx::Error>{
        let sql = format!("select * from users where ID={};",self.id);
        let res = sqlx::query(&sql)
            .fetch_all(pool)
            .await;
        match res {
            Ok(rows) => {
                if rows.len()>0 {
                    let user = UserInfo{ 
                        ID: rows[0].get("ID"),
                        PW: rows[0].get("PW"),
                        uName: rows[0].get("uName")
                    };
                    return Ok(user);
                }
                else {return Err(sqlx::Error::RowNotFound) };
            },
            Err(e) => {
                println!("sql:{}\n get database query bug",sql);
                return Err(e);
            }
        };
    }
}
